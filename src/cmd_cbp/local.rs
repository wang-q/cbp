use clap::*;

/// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("local")
        .about("Install packages from local binaries")
        .after_help(
            r###"
Install packages from local binaries or downloaded cache.

Search Locations:
* ./binaries/     - Pre-built binary directory (primary)
* ~/.cbp/cache/   - Downloaded packages (fallback)

Package Format:
* Naming: <package_name>.<os_type>.tar.gz
* Type: Pre-built binary archives
* OS: Platform-specific (macos/linux)

Features:
* Installation status checking
* Automatic location selection
* Package record management
* File extraction to ~/.cbp

Examples:
* Single package
  cbp local zlib

* Multiple packages
  cbp local zlib bzip2
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
            Arg::new("dir")
                .long("dir")
                .short('d')
                .num_args(1)
                .value_name("DIR")
                .help("Change working directory")
                .hide(true),
        )
}

/// Execute local command
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    let cbp_dirs = if args.contains_id("dir") {
        let home =
            std::path::Path::new(args.get_one::<String>("dir").unwrap()).to_path_buf();
        cbp::CbpDirs::from(home)?
    } else {
        cbp::CbpDirs::new()?
    };

    let os_type = cbp::get_os_type()?;

    // Process packages
    for pkg in args.get_many::<String>("packages").unwrap() {
        // Check if package is already installed
        let record_file = cbp_dirs.records.join(format!("{}.files", pkg));
        if record_file.exists() {
            println!("==> Package {} is already installed", pkg);
            continue;
        }

        // Try local binaries directory first
        let local_file =
            std::path::Path::new("binaries").join(format!("{}.{}.tar.gz", pkg, os_type));

        // Then try cache directory
        let cache_file = cbp_dirs.cache.join(format!("{}.{}.tar.gz", pkg, os_type));

        let pkg_file = if local_file.exists() {
            println!("==> Using locally built package from binaries/");
            local_file
        } else if cache_file.exists() {
            println!("==> Using cached package from ~/.cbp/cache/");
            cache_file
        } else {
            return Err(anyhow::anyhow!(
                "==> Package {}.{}.tar.gz not found in binaries/ or cache/",
                pkg,
                os_type
            ));
        };

        cbp::install_package(pkg, &pkg_file, &cbp_dirs)?;
    }

    Ok(())
}
