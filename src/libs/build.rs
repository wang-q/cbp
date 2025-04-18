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
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = std::fs::File::create(file_path)?;
    let resp = agent.get(url).call()?;
    std::io::copy(&mut resp.into_reader(), &mut file)?;
    Ok(())
}

/// Extract archive using custom command or default tar
/// Or copy single source file to destination
pub fn extract_archive(
    temp_dir: &tempfile::TempDir,
    temp_file: &std::path::Path,
    json_obj: &serde_json::Map<String, serde_json::Value>,
) -> anyhow::Result<()> {
    println!("-> Extracting archive");

    // Original archive extraction logic
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
    json_obj: &serde_json::Map<String, serde_json::Value>,
) -> anyhow::Result<()> {
    if let Some(rename) = json_obj.get("rename") {
        println!("  -> Processing rename rules");
        let rename_map = rename
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Rename must be an object"))?;

        // Only the first rename rule will be processed
        if let Some((pattern_str, target)) = rename_map.iter().next() {
            let target = target
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Rename target must be a string"))?;

            // Create parent directories if they don't exist
            if let Some(parent) = std::path::Path::new(target).parent() {
                std::fs::create_dir_all(temp_dir.path().join(parent))?;
            }

            // Use utils::match_files to find matching files
            let matches = crate::match_files(temp_dir.path(), pattern_str)?;
            if let Some((source_path, _)) = matches.first() {
                let target_path = temp_dir.path().join(target);
                if source_path != &target_path && source_path.exists() {
                    std::fs::rename(source_path, &target_path)?;
                    println!("    -> Renamed: {} -> {}", source_path.display(), target);
                }
            }
        }
    }
    Ok(())
}

/// Handle symlink creation based on package configuration
#[cfg(unix)]
pub fn handle_symlink(
    temp_dir: &tempfile::TempDir,
    json_obj: &serde_json::Map<String, serde_json::Value>,
) -> anyhow::Result<()> {
    if let Some(symlink) = json_obj.get("symlink") {
        println!("  -> Processing symlink rules");
        let symlink_map = symlink
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Symlink must be an object"))?;

        // Create bin directory for symlinks
        let bin_dir = temp_dir.path().join("bin");
        std::fs::create_dir_all(&bin_dir)?;

        // Process each symlink
        for (link_name, target) in symlink_map {
            let target = target
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Symlink target must be a string"))?;

            // Make target file executable
            let target_path = temp_dir.path().join(target);
            if target_path.exists() {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(
                    &target_path,
                    std::fs::Permissions::from_mode(0o755),
                )?;
            }

            let link_path = bin_dir.join(link_name);
            std::os::unix::fs::symlink(target, link_path)?;
        }
    }
    Ok(())
}

#[cfg(not(unix))]
pub fn handle_symlink(
    _temp_dir: &tempfile::TempDir,
    _json_obj: &serde_json::Map<String, serde_json::Value>,
) -> anyhow::Result<()> {
    Ok(())
}

/// Handle Windows shims
pub fn handle_shim(
    temp_dir: &tempfile::TempDir,
    json_obj: &serde_json::Map<String, serde_json::Value>,
) -> anyhow::Result<()> {
    if let Some(symlink) = json_obj.get("symlink") {
        println!("  -> Processing symlink rules");
        let symlink_map = symlink
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Symlink must be an object"))?;

        // Create bin directory for symlinks
        let bin_dir = temp_dir.path().join("bin");
        std::fs::create_dir_all(&bin_dir)?;

        // Process each symlink
        for (link_name, target) in symlink_map {
            let target = target
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Symlink target must be a string"))?;

            let script_path = bin_dir.join(format!("{}.ps1", link_name));

            // Create PowerShell wrapper script
            let script_content = format!("& \"$PSScriptRoot\\{}\" @args\n", target);
            std::fs::write(&script_path, script_content)?;
        }
    }
    Ok(())
}

pub fn handle_wrapper(
    temp_dir: &tempfile::TempDir,
    json_obj: &serde_json::Map<String, serde_json::Value>,
) -> anyhow::Result<()> {
    if let Some(wrapper) = json_obj.get("wrapper").and_then(|w| w.as_object()) {
        std::fs::create_dir_all(temp_dir.path().join("bin"))?;

        for (name, content) in wrapper {
            let script_path = temp_dir.path().join("bin").join(name);
            let content = match content {
                serde_json::Value::String(s) => s.to_string(),
                serde_json::Value::Array(lines) => {
                    lines
                        .iter()
                        .filter_map(|line| line.as_str())
                        .collect::<Vec<_>>()
                        .join("\n")
                        + "\n"
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "Wrapper content must be a string or array"
                    ))
                }
            };

            std::fs::write(&script_path, content)?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(
                    &script_path,
                    std::fs::Permissions::from_mode(0o755),
                )?;
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

            // find matching files
            let matches = crate::match_files(temp_dir.path(), path_str)?;
            for (path, rel_path) in matches {
                if path.is_dir() {
                    std::fs::remove_dir_all(&path)?;
                } else {
                    std::fs::remove_file(&path)?;
                }
                println!("    -> Removed: {}", rel_path);
            }
        }
    }
    Ok(())
}

pub fn find_binary_files(
    temp_dir: &std::path::Path,
    json_obj: &serde_json::Map<String, serde_json::Value>,
) -> anyhow::Result<Vec<String>> {
    let patterns: Vec<String> = match &json_obj["binary"] {
        serde_json::Value::String(pattern) => vec![pattern.to_string()],
        serde_json::Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect(),
        _ => return Err(anyhow::anyhow!("Invalid binary format")),
    };

    let mut binary_paths = Vec::new();
    for pattern in patterns {
        let matches = crate::libs::utils::match_files(temp_dir, &pattern)?;
        binary_paths.extend(matches.into_iter().map(|(_, rel_path)| rel_path));
    }

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
