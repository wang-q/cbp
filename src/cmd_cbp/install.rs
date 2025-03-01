use clap::*;

pub fn make_subcommand() -> Command {
    Command::new("install")
        .about("Download and install packages from GitHub")
        .after_help(
            r###"
Download and install packages from GitHub release repository.

Features:
* Installation status checking
* Automatic package downloading
* Platform-specific selection
* Proxy support for restricted networks
* Multiple package installation

[Package Source](https://github.com/wang-q/cbp/releases/tag/Binaries)

Examples:
* Single package
  cbp install zlib

* Multiple packages
  cbp install zlib bzip2

* With proxy
  cbp install --proxy socks5://127.0.0.1:7890 zlib
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
    let cbp_dirs = if args.contains_id("dir") {
        let home =
            std::path::Path::new(args.get_one::<String>("dir").unwrap()).to_path_buf();
        cbp::CbpDirs::from(home)?
    } else {
        cbp::CbpDirs::new()?
    };

    let os_type = cbp::get_os_type()?;

    // Setup HTTP client
    let agent = if let Some(proxy_url) = args.get_one::<String>("proxy") {
        let proxy = ureq::Proxy::new(proxy_url)?;
        ureq::AgentBuilder::new().proxy(proxy).build()
    } else {
        ureq::AgentBuilder::new().build()
    };

    // Process packages
    for pkg in args.get_many::<String>("packages").unwrap() {
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
        let url = format!(
            "https://github.com/wang-q/cbp/releases/download/Binaries/{}",
            pkg_file
        );
        let mut file = std::fs::File::create(&temp_file)?;
        let resp = agent.get(&url).call()?;
        std::io::copy(&mut resp.into_reader(), &mut file)?;

        // Move to final location
        std::fs::rename(&temp_file, &cache_file)?;

        // Install package
        cbp::install_package(pkg, &cache_file, &cbp_dirs)?;
    }

    Ok(())
}
