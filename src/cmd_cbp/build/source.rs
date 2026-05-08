use clap::*;
use cmd_lib::*;

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
    //----------------------------
    // Args
    //----------------------------
    // Get base directory
    let base_dir = std::path::PathBuf::from(args.get_one::<String>("base").unwrap());

    // Set up HTTP agent with optional proxy
    let opt_proxy_url = args.get_one::<String>("proxy");
    let agent = cbp::create_http_agent(opt_proxy_url)?;

    let cbp = std::env::current_exe()?.display().to_string();

    //----------------------------
    // Operating
    //----------------------------
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

        // Download file
        let url = dl_obj["url"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Source URL not found"))?;

        // Allow overriding the base URL via environment variable for testing
        let url = if let Some(path_part) = url.strip_prefix("https://github.com") {
            format!("{}{}", cbp::github_release_url(), path_part)
        } else {
            url.to_string()
        };

        let temp_file = cbp::temp_download_path(&temp_dir, dl_obj);

        // Process file after download
        println!("-> Downloading from {}", url);
        cbp::download_file(&url, &temp_file, &agent)?;

        let target_path = cbp::target_source_path(&base_dir, pkg)?;

        if dl_obj.len() == 1 {
            cbp::move_file_or_dir(&temp_file, std::path::Path::new(&target_path))?;
            println!("-> Successfully downloaded and processed");
            continue;
        }

        // Check if extraction is needed
        let needs_extract = cbp::needs_extract(&url, dl_obj);

        if needs_extract {
            println!("-> Processing source archive");
            cbp::extract_archive(&temp_dir, &temp_file, dl_obj)?;
        } else {
            cbp::normalize_line_endings(&temp_file)?;
        }

        let temp_path = temp_dir.path().canonicalize()?;

        cbp::handle_rename(&temp_dir, dl_obj)?;
        cbp::clean_files(&temp_dir, dl_obj)?;

        if temp_file.exists() {
            std::fs::remove_file(&temp_file)?;
        }

        run_cmd!(
            cd ${temp_path};
            ${cbp} tar . -o ${target_path}
        )?;
        println!("-> Successfully downloaded and processed");
    }

    Ok(())
}
