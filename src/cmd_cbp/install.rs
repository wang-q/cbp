use clap::*;

pub fn make_subcommand() -> Command {
    Command::new("install")
        .about("Download and install packages from GitHub")
        .after_help(include_str!("../../docs/help/install.md"))
        .arg(
            Arg::new("packages")
                .help("Package names to install")
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
                .help("Package type (font for fonts, default: platform specific)")
                .num_args(1)
                .value_name("TYPE")
                .value_parser(["macos", "linux", "windows", "font"]),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .num_args(1)
                .value_name("DIR")
                .help("Change working directory")
                .hide(true),
        )
}

pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Args
    //----------------------------
    let packages = args.get_many::<String>("packages").unwrap();

    // Set up HTTP agent with optional proxy
    let opt_proxy_url = args.get_one::<String>("proxy");
    let agent = cbp::create_http_agent(opt_proxy_url)?;

    let cbp_dirs = cbp::CbpDirs::from_arg_matches(args)?;

    let os_type = cbp::get_os_type()?;
    let pkg_type = args
        .get_one::<String>("type")
        .map(|s| s.as_str())
        .unwrap_or(&os_type);

    //----------------------------
    // Processing
    //----------------------------
    for pkg in packages {
        // Check if already installed
        let record_file = cbp_dirs.records.join(format!("{}.files", pkg));
        if record_file.exists() {
            println!("==> Package {} is already installed", pkg);
            continue;
        }

        // Download package
        println!("==> Downloading {}", pkg);
        let pkg_file = format!("{}.{}.tar.gz", pkg, pkg_type);
        let temp_file = cbp_dirs.cache.join(format!("{}.incomplete", pkg_file));
        let cache_file = cbp_dirs.cache.join(&pkg_file);

        // Create cache directory if needed
        std::fs::create_dir_all(&cbp_dirs.cache)?;

        // Download from GitHub
        let base_url = cbp::github_release_url();
        let url = format!(
            "{}/wang-q/cbp/releases/download/Binaries/{}",
            base_url, pkg_file
        );
        let mut file = std::fs::File::create(&temp_file)?;
        let resp = agent.get(&url).call()?;
        std::io::copy(&mut resp.into_reader(), &mut file)?;

        // Move to final location using move_file_or_dir to handle cross-device scenarios
        cbp::move_file_or_dir(&temp_file, &cache_file)?;

        // Install package
        cbp_dirs.install_package(pkg, &cache_file)?;
        println!("==> Successfully installed {}", pkg);
    }

    // Font installation reminder
    if pkg_type == "font" {
        println!("==> Fonts installed to ~/.cbp/share/fonts");
        print!(
            "{}",
            cbp::font_install_instructions(&os_type, &cbp_dirs.home.join("share/fonts"))
        );
    }

    Ok(())
}
