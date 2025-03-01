use clap::*;

/// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("local")
        .about("Install packages from local binaries")
        .after_help(
            r###"
Install packages from local binaries directory.

The command looks for package files in ~/.cbp/cache/ with the format:
  <package_name>.<os_type>.tar.gz

Examples:
1. Install a single package:
   cbp local zlib

2. Install multiple packages:
   cbp local zlib bzip2

Note: Package files must exist in the binaries/ directory.
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

    let os_type = cbp::libs::utils::get_os_type()?;

    // Process packages
    for pkg in args.get_many::<String>("packages").unwrap() {
        let pkg_file = cbp_dirs.records.join(format!("{}.{}.tar.gz", pkg, os_type));

        if !pkg_file.exists() {
            return Err(anyhow::anyhow!(
                "==> Package {}.{}.tar.gz not found in binaries/",
                pkg,
                os_type
            ));
        }

        // Check if package is already installed
        let record_file = cbp_dirs.records.join(format!("{}.files", pkg));
        if record_file.exists() {
            println!("==> Package {} is already installed", pkg);
            continue;
        }

        cbp::libs::utils::install_package(pkg, &pkg_file, &cbp_dirs)?;
    }

    Ok(())
}
