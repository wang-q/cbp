use clap::*;
use cmd_lib::*;

/// Create clap subcommand arguments
pub fn make_subcommand() -> clap::Command {
    clap::Command::new("prebuild")
        .about("Build prebuild packages")
        .arg(
            Arg::new("packages")
                .help("Package names to build")
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
            Arg::new("type")
                .long("type")
                .short('t')
                .help("Package type")
                .num_args(1)
                .value_name("TYPE")
                .value_parser(["linux", "macos", "windows"]),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .help("Base directory containing packages/ and binaries/")
                .num_args(1)
                .value_name("DIR")
                .default_value("."),
        )
}

/// Execute prebuild subcommand
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Args
    //----------------------------
    let base_dir = std::path::PathBuf::from(args.get_one::<String>("dir").unwrap());
    let opt_proxy_url = args.get_one::<String>("proxy");
    let agent = cbp::create_http_agent(opt_proxy_url)?;

    let cbp = std::env::current_exe()?.display().to_string();

    // Get specified OS type
    let opt_type = args.get_one::<String>("type");

    //----------------------------
    // Operating
    //----------------------------
    for pkg in args.get_many::<String>("packages").unwrap() {
        println!("==> Processing prebuild package: {}", pkg);

        // Read package configuration
        let json = cbp::read_package_json(&base_dir, pkg)?;

        // Process each available OS type
        let downloads = json["downloads"]
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Downloads configuration not found"))?;

        for (os_type, download_obj) in downloads {
            // Skip if type is specified and doesn't match
            if let Some(type_filter) = opt_type {
                if type_filter != os_type {
                    continue;
                }
            } else if !["linux", "macos", "windows"].contains(&os_type.as_str()) {
                continue;
            }

            let dl_obj = download_obj.as_object().ok_or_else(|| {
                anyhow::anyhow!("Download configuration not found for {}", os_type)
            })?;

            println!("-> Processing for OS: {}", os_type);

            let temp_dir = tempfile::tempdir()?;
            let temp_file = temp_dir.path().join("download.tmp");

            // Download file
            let url = dl_obj["url"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("URL not found"))?;
            println!("-> Downloading from {}", url);
            cbp::download_file(url, &temp_file, &agent)?;

            // Check if extraction is needed
            let needs_extract = url.ends_with(".zip")
                || url.ends_with(".tar.gz")
                || dl_obj.get("extract").is_some();

            if needs_extract {
                // Process downloaded file
                cbp::extract_archive(&temp_dir, &temp_file, dl_obj)?;
                cbp::handle_rename(&temp_dir, dl_obj)?;
                if os_type != "windows" {
                    cbp::handle_symlink(&temp_dir, dl_obj)?;
                }
                cbp::clean_files(&temp_dir, dl_obj)?;
                std::fs::remove_file(&temp_file)?;
            } else {
                // For single binary files, just rename the downloaded file
                let binary_name = dl_obj["binary"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Binary name not found"))?;
                std::fs::rename(&temp_file, temp_dir.path().join(binary_name))?;
            }

            // Create final package
            std::fs::create_dir_all(base_dir.join("binaries"))?;
            let target_path = base_dir
                .canonicalize()?
                .join("binaries")
                .join(format!("{}.{}.tar.gz", pkg, os_type))
                .display()
                .to_string();
            let temp_path = temp_dir.path().canonicalize()?;

            // Add shebang option if enabled
            let shebang_opt =
                if dl_obj.get("shebang").and_then(|v| v.as_bool()) == Some(true) {
                    "--shebang"
                } else {
                    ""
                };

            // Only process binary files if binary configuration exists
            if dl_obj.get("binary").is_some() {
                // Find binary files
                let binary_paths = cbp::find_binary_files(temp_dir.path(), dl_obj)?;

                // Set executable permissions for binary files
                #[cfg(unix)]
                for binary_path in &binary_paths {
                    use std::os::unix::fs::PermissionsExt;
                    let full_path = temp_dir.path().join(binary_path);
                    std::fs::set_permissions(
                        &full_path,
                        std::fs::Permissions::from_mode(0o755),
                    )?;
                }

                // Change to temp directory and collect files
                run_cmd!(
                    cd ${temp_path};
                    ${cbp} collect --mode bin ${shebang_opt} -o ${target_path} $[binary_paths]
                )?;
            } else {
                // Change to temp directory and collect files
                // cbp collect can't handle symlinks
                run_cmd!(
                    cd ${temp_path};
                    ${cbp} tar . -o ${target_path}
                )?;
            }
            println!("-> Package created successfully");
        }
    }

    Ok(())
}
