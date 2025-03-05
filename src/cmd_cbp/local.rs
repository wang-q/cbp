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
* OS: Platform-specific (linux/macos/windows)

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

Developer Options:
* Install cross-platform packages (use with caution)
  cbp local --type windows zlib

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
            Arg::new("type")
                .long("type")
                .short('t')
                .num_args(1)
                .value_name("OS_TYPE")
                .help("Install packages for specified OS")
                .value_parser(["macos", "linux", "windows"]),
        )
        .arg(
            Arg::new("list")
                .long("list")
                .short('l')
                .help("List contents of packages without installing")
                .action(ArgAction::SetTrue),
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

    let os_type = if args.contains_id("type") {
        args.get_one::<String>("type").unwrap().to_string()
    } else {
        cbp::get_os_type()?
    };

    let list_only = args.get_flag("list");

    // Process packages
    for pkg in args.get_many::<String>("packages").unwrap() {
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

        if list_only {
            println!("==> Contents of package {}:", pkg);
            let contents = cbp::list_archive_files(&pkg_file)?;
            print!("{}", contents);
            continue;
        }

        // Check if package is already installed
        let record_file = cbp_dirs.records.join(format!("{}.files", pkg));
        if record_file.exists() {
            println!("==> Package {} is already installed", pkg);
            continue;
        }

        cbp_dirs.install_package(pkg, &pkg_file)?;
    }

    Ok(())
}
