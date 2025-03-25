use clap::*;
use glob::Pattern;
use std::path::Path;

/// Create clap subcommand arguments
pub fn make_subcommand() -> clap::Command {
    clap::Command::new("download")
        .about("Download package sources")
        .arg(
            Arg::new("packages")
                .help("Package names to download")
                .required(true)
                .num_args(1..)
                .value_name("PACKAGES"),
        )
        .arg(
            Arg::new("proxy")
                .long("proxy")
                .help("Proxy server URL (e.g., socks5://127.0.0.1:7890)")
                .num_args(1)
                .value_name("URL"),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .help("Base directory containing packages/ and sources/")
                .num_args(1)
                .value_name("DIR")
                .default_value("."),
        )
}

/// Execute download subcommand
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    // Get base directory
    let base_dir = std::path::PathBuf::from(args.get_one::<String>("dir").unwrap());

    // Set up HTTP agent with optional proxy
    let opt_proxy_url = args.get_one::<String>("proxy");
    let agent = cbp::create_http_agent(opt_proxy_url)?;

    // Process packages
    for pkg in args.get_many::<String>("packages").unwrap() {
        download_package(&base_dir, pkg, &agent)?;
    }

    Ok(())
}

/// Download and process a single package
/// 
/// # Arguments
/// * `base_dir` - Base directory containing packages/ and sources/
/// * `pkg` - Package name
/// * `agent` - HTTP agent for downloading
fn download_package(
    base_dir: &Path,
    pkg: &str,
    agent: &ureq::Agent,
) -> anyhow::Result<()> {
    println!("==> Processing package: {}", pkg);
    let json = read_package_json(base_dir, pkg)?;
    let source_url = get_source_url(&json)?;
    let temp_dir = tempfile::tempdir()?;
    let temp_file = temp_dir.path().join("download.tmp");

    println!("-> Downloading from {}", source_url);
    download_source(&source_url, &temp_file, agent)?;
    if let serde_json::Value::Object(source_obj) = &json["source"] {
        println!("-> Processing source archive");
        process_source_object(&temp_dir, &temp_file, source_obj, pkg)?;
    }

    finalize_download(base_dir, pkg, &temp_file)?;
    println!("-> Successfully downloaded and processed");
    Ok(())
}

/// Read and validate package JSON configuration
/// 
/// # Returns
/// * JSON object containing package configuration
/// * Error if package file not found or validation fails
fn read_package_json(base_dir: &Path, pkg: &str) -> anyhow::Result<serde_json::Value> {
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

/// Extract source URL from package JSON
/// 
/// Handles both string format and object format with url field
/// Supports GITHUB_RELEASE_URL environment variable override
fn get_source_url(json: &serde_json::Value) -> anyhow::Result<String> {
    let url = match &json["source"] {
        serde_json::Value::String(url) => url.to_string(),
        serde_json::Value::Object(obj) => obj["url"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Source URL not found"))?
            .to_string(),
        _ => return Err(anyhow::anyhow!("Invalid source format")),
    };

    // Handle GitHub URL override
    if let Ok(github_url) = std::env::var("GITHUB_RELEASE_URL") {
        if url.starts_with("https://github.com") {
            return Ok(url.replace("https://github.com", &github_url));
        }
    }

    Ok(url)
}

fn download_source(
    url: &str,
    file_path: &Path,
    agent: &ureq::Agent,
) -> anyhow::Result<()> {
    let mut file = std::fs::File::create(file_path)?;
    let resp = agent.get(url).call()?;
    std::io::copy(&mut resp.into_reader(), &mut file)?;
    Ok(())
}

/// Process source object after download
/// 
/// Steps:
/// 1. Extract archive
/// 2. Handle rename
/// 3. Clean files
/// 4. Create reproducible archive
fn process_source_object(
    temp_dir: &tempfile::TempDir,
    temp_file: &Path,
    source_obj: &serde_json::Map<String, serde_json::Value>,
    pkg: &str,
) -> anyhow::Result<()> {
    extract_archive(temp_dir, temp_file, source_obj)?;
    let rename_target = handle_rename(temp_dir, source_obj, pkg)?;
    clean_files(temp_dir, source_obj)?;
    create_reproducible_archive(temp_dir, temp_file, &rename_target)?;
    Ok(())
}

/// Extract archive using custom command or default tar
/// 
/// Uses gtar on macOS and tar on other platforms
fn extract_archive(
    temp_dir: &tempfile::TempDir,
    temp_file: &Path,
    source_obj: &serde_json::Map<String, serde_json::Value>,
) -> anyhow::Result<()> {
    println!("-> Extracting archive");
    if let Some(extract_cmd) = source_obj.get("extract") {
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
/// 
/// If rename is not specified, uses package name as target
fn handle_rename(
    temp_dir: &tempfile::TempDir,
    source_obj: &serde_json::Map<String, serde_json::Value>,
    pkg: &str,
) -> anyhow::Result<String> {
    if let Some(rename) = source_obj.get("rename") {
        println!("  -> Processing rename rules");
        let rename_map = rename
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Rename must be an object"))?;
        let result = handle_rename_map(temp_dir.path(), rename_map)?;
        println!("  -> Renamed to: {}", result);
        Ok(result)
    } else {
        println!("  -> Using package name as target: {}", pkg);
        Ok(pkg.to_string())
    }
}

/// Clean files matching patterns specified in package configuration
fn clean_files(
    temp_dir: &tempfile::TempDir,
    source_obj: &serde_json::Map<String, serde_json::Value>,
) -> anyhow::Result<()> {
    if let Some(clean) = source_obj.get("clean") {
        let clean_paths = clean
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Clean must be an array"))?;
        
        println!("  -> Cleaning {} patterns", clean_paths.len());
        for path in clean_paths {
            let path_str = path
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Clean path must be a string"))?;
            let pattern = Pattern::new(path_str)?;

            // Recursively walk through the directory to match all files
            for entry in walkdir::WalkDir::new(temp_dir.path()) {
                let entry = entry?;
                let rel_path = entry.path().strip_prefix(temp_dir.path())?.to_string_lossy();
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

/// Create reproducible tar archive with standardized attributes
/// 
/// Sets consistent ownership, permissions, and timestamps
fn create_reproducible_archive(
    temp_dir: &tempfile::TempDir,
    temp_file: &Path,
    rename_target: &str,
) -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new(if cfg!(target_os = "macos") {
        "gtar"
    } else {
        "tar"
    });
    
    // Set GZIP=-n environment variable
    cmd.env("GZIP", "-n")
        .args([
            "--format=gnu",
            "--sort=name",
            "--owner=0",
            "--group=0",
            "--numeric-owner",
            "--mode=a+rX,u+w,go-w",
            "--mtime=2024-01-01 00:00Z",
            "-czf",
        ])
        .arg(temp_file)
        .arg(rename_target)
        .current_dir(temp_dir.path())
        .status()?;
    
    Ok(())
}

/// Move downloaded and processed file to final location
fn finalize_download(
    base_dir: &Path,
    pkg: &str,
    temp_file: &Path,
) -> anyhow::Result<()> {
    let target_path = base_dir.join("sources").join(format!("{}.tar.gz", pkg));
    std::fs::create_dir_all(base_dir.join("sources"))?;
    std::fs::rename(temp_file, target_path)?;
    Ok(())
}

/// Find first directory in temp_dir and return its name
/// 
/// Used as fallback when rename target is not specified
fn get_first_directory(temp_dir: &std::path::Path) -> anyhow::Result<String> {
    let entries: Vec<_> = std::fs::read_dir(temp_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();

    if let Some(first_dir) = entries.first() {
        let target = first_dir.file_name().to_string_lossy().into_owned();
        println!("-> Using first directory as target: {}", target);
        Ok(target)
    } else {
        Err(anyhow::anyhow!("No directory found for rename target"))
    }
}

/// Handle rename map from package configuration
/// 
/// Supports glob patterns for matching source directories
fn handle_rename_map(
    temp_dir: &std::path::Path,
    rename_map: &serde_json::Map<String, serde_json::Value>,
) -> anyhow::Result<String> {
    if let Some((pattern_str, target)) = rename_map.iter().next() {
        let target = target
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Rename target must be a string"))?;

        let pattern = Pattern::new(pattern_str)?;

        // Find matching files
        let entries: Vec<_> = std::fs::read_dir(temp_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| pattern.matches(&e.file_name().to_string_lossy()))
            .collect();

        if let Some(entry) = entries.first() {
            let source = entry.file_name();
            if source.to_string_lossy() != target {
                std::fs::rename(temp_dir.join(&source), temp_dir.join(target))?;
            }
            Ok(target.to_string())
        } else {
            get_first_directory(temp_dir)
        }
    } else {
        get_first_directory(temp_dir)
    }
}
