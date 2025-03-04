use clap::*;

pub fn make_subcommand() -> Command {
    Command::new("remove")
        .about("Remove installed packages")
        .after_help(
            r###"
Remove installed packages and their associated files.

This command safely removes installed packages and their associated files,
including resource forks and symbolic links. It preserves directory structure
and skips missing files to maintain system integrity. The removal process is
guided by package records to ensure only managed files are affected, leaving
other files untouched.

Examples:
* Single package
  cbp remove zlib

* Multiple packages
  cbp remove zlib bzip2

Warning: This operation cannot be undone.

"###,
        )
        .arg(
            Arg::new("packages")
                .help("Name of the packages")
                .num_args(1..)
                .required(true)
                .index(1)
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

pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    let cbp_dirs = if args.contains_id("dir") {
        let home =
            std::path::Path::new(args.get_one::<String>("dir").unwrap()).to_path_buf();
        cbp::CbpDirs::from(home)?
    } else {
        cbp::CbpDirs::new()?
    };

    for package in args.get_many::<String>("packages").unwrap() {
        let file_path = cbp_dirs.records.join(format!("{}.files", package));
        if !file_path.exists() {
            println!("==> Package {} is not installed", package);
            continue;
        }

        println!("==> Removing {}:", package);
        let content = std::fs::read_to_string(&file_path)?;

        for line in content.lines() {
            if line.is_empty() {
                continue;
            }

            let file = cbp_dirs.home.join(line);
            if !file.exists() && !file.is_symlink() {
                println!("    File not found: {}", file.display());
                continue;
            }

            if !file.is_file() && !file.is_symlink() {
                continue;
            }

            std::fs::remove_file(&file)?;

            // Handle resource fork files
            let file_name = match file.file_name().and_then(|n| n.to_str()) {
                Some(name) => name,
                None => continue,
            };

            let resource_fork = file.parent().unwrap().join(format!("._{}", file_name));
            if resource_fork.exists() {
                std::fs::remove_file(&resource_fork)?;
            }
        }

        std::fs::remove_file(&file_path)?;
        println!("    Done");
    }

    Ok(())
}
