use clap::*;

pub fn make_subcommand() -> Command {
    Command::new("install")
        .about("Download and install packages from GitHub")
        .after_help(
            r###"
Download and install packages from GitHub release repository.

This command downloads and installs pre-built binary packages from GitHub.
It checks for existing installations to avoid duplicates and handles
platform-specific package selection automatically.

[Package Source](https://github.com/wang-q/cbp/releases/tag/Binaries)

Examples:
* Basic usage
  cbp install zlib            # single package
  cbp install zlib bzip2      # multiple packages

* Network proxy support
  # Priority (high to low):
  # 1. --proxy argument
  cbp install --proxy socks5://127.0.0.1:7890 zlib
  # 2. Environment variables:
  #    ALL_PROXY
  #    HTTP_PROXY
  #    all_proxy
  #    http_proxy
"###,
        )
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

    let cbp_dirs = if args.contains_id("dir") {
        let home =
            std::path::Path::new(args.get_one::<String>("dir").unwrap()).to_path_buf();
        cbp::CbpDirs::from(home)?
    } else {
        cbp::CbpDirs::new()?
    };

    let os_type = cbp::get_os_type()?;

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
        let pkg_file = format!("{}.{}.tar.gz", pkg, os_type);
        let temp_file = cbp_dirs.cache.join(format!("{}.incomplete", pkg_file));
        let cache_file = cbp_dirs.cache.join(&pkg_file);

        // Create cache directory if needed
        std::fs::create_dir_all(&cbp_dirs.cache)?;

        // Download from GitHub
        let base_url = std::env::var("GITHUB_RELEASE_URL")
            .unwrap_or_else(|_| "https://github.com".to_string());
        let url = format!(
            "{}/wang-q/cbp/releases/download/Binaries/{}",
            base_url, pkg_file
        );
        let mut file = std::fs::File::create(&temp_file)?;
        let resp = agent.get(&url).call()?;
        std::io::copy(&mut resp.into_reader(), &mut file)?;

        // Move to final location
        std::fs::rename(&temp_file, &cache_file)?;

        // Install package
        cbp_dirs.install_package(pkg, &cache_file)?;
    }

    Ok(())
}
