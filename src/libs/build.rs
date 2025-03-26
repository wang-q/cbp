use std::io::Read;

/// Read and validate package JSON configuration
///
/// # Returns
/// * JSON object containing package configuration
/// * Error if package file not found or validation fails
pub fn read_package_json(
    base_dir: &std::path::Path,
    pkg: &str,
) -> anyhow::Result<serde_json::Value> {
    let json_path = base_dir.join("packages").join(format!("{}.json", pkg));
    if !json_path.exists() {
        return Err(anyhow::anyhow!(
            "Package file {} not found",
            json_path.display()
        ));
    }

    let json_content = std::fs::read_to_string(&json_path)?;
    let json: serde_json::Value = serde_json::from_str(&json_content)?;

    // Validate package name
    let name = json["name"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Field 'name' not found"))?;
    if name != pkg {
        return Err(anyhow::anyhow!(
            "Package name in JSON ({}) does not match requested package ({})",
            name,
            pkg
        ));
    }

    Ok(json)
}

pub fn download_file(
    url: &str,
    file_path: &std::path::Path,
    agent: &ureq::Agent,
) -> anyhow::Result<()> {
    let mut file = std::fs::File::create(file_path)?;
    let resp = agent.get(url).call()?;
    std::io::copy(&mut resp.into_reader(), &mut file)?;
    Ok(())
}

/// Extract archive using custom command or default tar
///
/// Uses gtar on macOS and tar on other platforms
pub fn extract_archive(
    temp_dir: &tempfile::TempDir,
    temp_file: &std::path::Path,
    json_obj: &serde_json::Map<String, serde_json::Value>,
) -> anyhow::Result<()> {
    println!("-> Extracting archive");
    if let Some(extract_cmd) = json_obj.get("extract") {
        let cmd_str = extract_cmd
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Extract command must be a string"))?;
        println!("-> Using custom extract command: {}", cmd_str);

        let mut parts = cmd_str.split_whitespace();
        let program = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Empty extract command"))?;

        std::process::Command::new(program)
            .args(parts)
            .arg(temp_file)
            .current_dir(temp_dir.path())
            .status()?;
    } else {
        std::process::Command::new(if cfg!(target_os = "macos") {
            "gtar"
        } else {
            "tar"
        })
        .arg("xf")
        .arg(temp_file)
        .current_dir(temp_dir.path())
        .status()?;
    }
    println!("  -> Extraction completed");
    Ok(())
}

/// Handle file renaming based on package configuration
pub fn handle_rename(
    temp_dir: &tempfile::TempDir,
    source_obj: &serde_json::Map<String, serde_json::Value>,
) -> anyhow::Result<()> {
    if let Some(rename) = source_obj.get("rename") {
        println!("  -> Processing rename rules");
        let rename_map = rename
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Rename must be an object"))?;

        // Only the first rename rule will be processed
        // Multiple renames should be handled by array of rules
        if let Some((pattern_str, target)) = rename_map.iter().next() {
            let target = target
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Rename target must be a string"))?;

            let pattern = glob::Pattern::new(pattern_str)?;

            // Find matching files
            let entries: Vec<_> = std::fs::read_dir(temp_dir.path())?
                .filter_map(|e| e.ok())
                .filter(|e| pattern.matches(&e.file_name().to_string_lossy()))
                .collect();

            if let Some(entry) = entries.first() {
                let source = entry.file_name();
                if source.to_string_lossy() != target {
                    std::fs::rename(temp_dir.path().join(&source), temp_dir.path().join(target))?;
                }
            }
        }
    }
    Ok(())
}

/// Clean files matching patterns specified in package configuration
pub fn clean_files(
    temp_dir: &tempfile::TempDir,
    json_obj: &serde_json::Map<String, serde_json::Value>,
) -> anyhow::Result<()> {
    if let Some(clean) = json_obj.get("clean") {
        let clean_paths = clean
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Clean must be an array"))?;

        println!("  -> Cleaning {} patterns", clean_paths.len());
        for path in clean_paths {
            let path_str = path
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Clean path must be a string"))?;
            let pattern = glob::Pattern::new(path_str)?;

            // Recursively walk through the directory to match all files
            for entry in walkdir::WalkDir::new(temp_dir.path()) {
                let entry = entry?;
                let rel_path = entry
                    .path()
                    .strip_prefix(temp_dir.path())?
                    .to_string_lossy();
                if pattern.matches(&rel_path) {
                    let path = entry.path();
                    if path.is_dir() {
                        std::fs::remove_dir_all(&path)?;
                    } else {
                        std::fs::remove_file(&path)?;
                    }
                    println!("    -> Removed: {}", rel_path);
                }
            }
        }
    }
    Ok(())
}

pub fn find_binary_files(
    temp_dir: &std::path::Path,
    json_obj: &serde_json::Map<String, serde_json::Value>,
) -> anyhow::Result<Vec<String>> {
    let binary_paths = match &json_obj["binary"] {
        serde_json::Value::String(pattern) => {
            let mut paths = Vec::new();
            for entry in glob::glob(&format!("{}/{}", temp_dir.display(), pattern))? {
                paths.push(entry?.file_name().unwrap().to_string_lossy().to_string());
            }
            paths
        }
        serde_json::Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect(),
        _ => return Err(anyhow::anyhow!("Invalid binary format")),
    };

    if binary_paths.is_empty() {
        return Err(anyhow::anyhow!("No binary files found"));
    }

    Ok(binary_paths)
}

pub fn fix_shebang(path: &std::path::Path) -> anyhow::Result<()> {
    // Check if it's a text file
    let mut file = std::fs::File::open(path)?;
    let mut buffer = [0u8; 512];
    let n = file.read(&mut buffer)?;

    // Check if the first n bytes are valid UTF-8 or ASCII characters
    if !buffer[..n]
        .iter()
        .all(|&b| b.is_ascii() || (b & 0xC0) == 0x80)
    {
        return Ok(());
    }

    // Read file content
    let content = std::fs::read_to_string(path)?;
    let mut lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return Ok(());
    }

    // Check for shebang line
    if !lines[0].starts_with("#!") {
        return Ok(());
    }

    // Fix shebang line
    let first_line = lines[0];
    let new_line = if first_line.contains("perl") {
        "#!/usr/bin/env perl"
    } else if first_line.contains("python") {
        "#!/usr/bin/env python3"
    } else {
        return Ok(());
    };

    if first_line != new_line {
        lines[0] = new_line;
        let new_content = lines.join("\n") + "\n";
        std::fs::write(path, new_content)?;
        eprintln!("==> Fixed shebang in '{}'", path.display());
    }

    Ok(())
}
