use clap::*;

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
            Arg::new("base")
                .long("base")
                .help("Base directory containing packages/ and sources/")
                .num_args(1)
                .value_name("BASE")
                .default_value("."),
        )
}

/// Execute download subcommand
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    // Get base directory
    let base_dir = std::path::PathBuf::from(args.get_one::<String>("base").unwrap());

    // Set up HTTP agent with optional proxy
    let opt_proxy_url = args.get_one::<String>("proxy");
    let agent = cbp::create_http_agent(opt_proxy_url)?;

    // Process packages
    for pkg in args.get_many::<String>("packages").unwrap() {
        println!("==> Processing source package: {}", pkg);

        // Read and validate package configuration
        let json = cbp::read_package_json(&base_dir, pkg)?;

        // Get source download configuration
        let dl_obj = json["downloads"]["source"]
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Download configuration not found"))?;

        let temp_dir = tempfile::tempdir()?;
        let temp_file = temp_dir.path().join("download.tmp");

        // Download font file
        let url = dl_obj["url"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Font URL not found"))?;
        println!("-> Downloading from {}", url);
        cbp::download_file(url, &temp_file, &agent)?;

        // Check number of keys in dl_obj
        if dl_obj.len() > 1 {
            println!("-> Processing source archive");
            cbp::extract_archive(&temp_dir, &temp_file, dl_obj)?;
            cbp::handle_rename(&temp_dir, dl_obj)?;
            cbp::clean_files(&temp_dir, dl_obj)?;

            let target_name = get_target_name(&temp_dir, dl_obj, pkg)?;
            create_reproducible_archive(&temp_dir, &temp_file, &target_name)?;
        }

        let target_path = base_dir.join("sources").join(format!("{}.tar.gz", pkg));
        std::fs::create_dir_all(base_dir.join("sources"))?;
        std::fs::rename(temp_file, target_path)?;

        println!("-> Successfully downloaded and processed");
    }

    Ok(())
}

/// Get target directory name for archive
fn get_target_name(
    temp_dir: &tempfile::TempDir,
    source_obj: &serde_json::Map<String, serde_json::Value>,
    pkg: &str,
) -> anyhow::Result<String> {
    if let Some(rename) = source_obj.get("rename") {
        let rename_map = rename
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Rename must be an object"))?;
        if let Some(target) = get_rename_target(rename_map)? {
            println!("  -> Using rename target: {}", target);
            return Ok(target);
        }
    }

    // Fallback to first directory or package name
    if let Ok(first_dir) = get_first_directory(temp_dir.path()) {
        Ok(first_dir)
    } else {
        println!("  -> Using package name as target: {}", pkg);
        Ok(pkg.to_string())
    }
}

/// Get target name from rename map
fn get_rename_target(
    rename_map: &serde_json::Map<String, serde_json::Value>,
) -> anyhow::Result<Option<String>> {
    if let Some((_, target)) = rename_map.iter().next() {
        let target = target
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Rename target must be a string"))?;
        Ok(Some(target.to_string()))
    } else {
        Ok(None)
    }
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

/// Create reproducible tar archive with standardized attributes
///
/// Sets consistent ownership, permissions, and timestamps
fn create_reproducible_archive(
    temp_dir: &tempfile::TempDir,
    temp_file: &std::path::Path,
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
