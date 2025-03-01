use clap::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("list")
        .about("List installed package(s)")
        .after_help(
            r###"

"###,
        )
        .arg(
            Arg::new("packages")
                .help("Name of the packages")
                .num_args(0..)
                .index(1),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .num_args(1)
                .value_name("DIR")
                .help("Change working directory"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    let cbp_dirs = if args.contains_id("dir") {
        let home =
            std::path::Path::new(args.get_one::<String>("dir").unwrap()).to_path_buf();
        cbp::CbpDirs::from(home)?
    } else {
        cbp::CbpDirs::new()?
    };

    if let Some(packages) = args.get_many::<String>("packages") {
        for package in packages {
            let file_path = cbp_dirs.binaries.join(format!("{}.files", package));
            if file_path.exists() {
                println!("==> Files in package {}:", package);
                let content = std::fs::read_to_string(&file_path)?;
                print!("{}", content);
                println!();
            } else {
                println!("Warning: Package {} is not installed", package);
            }
        }
    } else {
        println!("==> Installed packages:");
        if cbp_dirs.binaries.exists() {
            let files = cbp::find_files(&cbp_dirs.binaries, Some("*.files"))?;
            let packages: Vec<String> = files
                .iter()
                .filter_map(|f| f.strip_suffix(".files").map(|s| s.to_string()))
                .collect();

            if !packages.is_empty() {
                println!("{}", cbp::format_packages(&packages));
            }
        }
        println!();
    }

    Ok(())
}
