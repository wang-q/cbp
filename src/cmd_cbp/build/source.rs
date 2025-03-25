use clap::*;
use glob::Pattern;
use std::path::Path;

/// Create clap subcommand arguments
pub fn make_subcommand() -> clap::Command {
    clap::Command::new("source")
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
        println!("==> Processing source package: {}", pkg);

        // Read and validate package configuration
        let json = cbp::read_package_json(&base_dir, pkg)?;

        let temp_dir = tempfile::tempdir()?;
        let temp_file = temp_dir.path().join("download.tmp");

        let source_url = get_source_url(&json)?;
        println!("-> Downloading from {}", source_url);
        cbp::download_file(&source_url, &temp_file, &agent)?;

        if let serde_json::Value::Object(source_obj) = &json["source"] {
            println!("-> Processing source archive");
            cbp::extract_archive(&temp_dir, &temp_file, source_obj)?;
            let rename_target = handle_rename(&temp_dir, source_obj, pkg)?;
            cbp::clean_files(&temp_dir, source_obj)?;
            create_reproducible_archive(&temp_dir, &temp_file, &rename_target)?;
        }

        let target_path = base_dir.join("sources").join(format!("{}.tar.gz", pkg));
        std::fs::create_dir_all(base_dir.join("sources"))?;
        std::fs::rename(temp_file, target_path)?;

        println!("-> Successfully downloaded and processed");
    }

    Ok(())
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
