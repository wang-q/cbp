use clap::*;
use cmd_lib::*;

/// Create clap subcommand arguments
pub fn make_subcommand() -> clap::Command {
    clap::Command::new("font")
        .about("Build font packages")
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
            Arg::new("dir")
                .long("dir")
                .short('d')
                .help("Base directory containing packages/ and binaries/")
                .num_args(1)
                .value_name("DIR")
                .default_value("."),
        )
}

/// Execute font subcommand
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Args
    //----------------------------
    let base_dir = std::path::PathBuf::from(args.get_one::<String>("dir").unwrap());
    let opt_proxy_url = args.get_one::<String>("proxy");
    let agent = cbp::create_http_agent(opt_proxy_url)?;

    let cbp = std::env::current_exe()?.display().to_string();

    //----------------------------
    // Operating
    //----------------------------
    for pkg in args.get_many::<String>("packages").unwrap() {
        println!("==> Processing font package: {}", pkg);

        // Read and validate package configuration
        let json = cbp::read_package_json(&base_dir, pkg)?;

        // Ensure it's a font package
        if json["type"].as_str() != Some("font") {
            return Err(anyhow::anyhow!("Package {} is not a font package", pkg));
        }

        // Get download configuration
        let dl_obj = json["downloads"]["font"]
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

        // Process downloaded file
        cbp::extract_archive(&temp_dir, &temp_file, dl_obj)?;
        cbp::clean_files(&temp_dir, dl_obj)?;

        // Find binary files
        let binary_paths = cbp::find_binary_files(temp_dir.path(), dl_obj)?;

        // Create final font package
        let target_path = base_dir
            .canonicalize()?
            .join("binaries")
            .join(format!("{}.font.tar.gz", pkg))
            .display()
            .to_string();
        let temp_path = temp_dir.path().canonicalize()?;

        // Create binaries directory if it doesn't exist
        std::fs::create_dir_all(base_dir.join("binaries"))?;

        // Change to temp directory and collect files
        run_cmd!(
            cd ${temp_path};
            ${cbp} collect --mode font -o ${target_path} $[binary_paths]
        )?;

        println!("-> Font package created successfully");
    }

    Ok(())
}
