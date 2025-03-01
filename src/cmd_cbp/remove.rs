use clap::*;

pub fn make_subcommand() -> Command {
    Command::new("remove")
        .about("Remove installed package(s)")
        .after_help(
            r###"
Remove installed packages and their files.

The command will:
* Remove all files installed by the package
* Remove package record file
* Skip directories (they will be kept)
* Handle both regular files and symbolic links

Examples:
1. Remove a single package:
   cbp remove zlib

2. Remove multiple packages:
   cbp remove zlib bzip2

Note: This command cannot be undone. Please make sure you want to remove these packages.
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
        let file_path = cbp_dirs.binaries.join(format!("{}.files", package));
        if !file_path.exists() {
            println!("Warning: Package {} is not installed", package);
            continue;
        }

        println!("==> Removing {}", package);

        // Remove package files
        let content = std::fs::read_to_string(&file_path)?;
        for file in content.lines() {
            let path = cbp_dirs.home.join(file);
            // Skip if it's a directory
            if path.is_dir() {
                continue;
            }
            if path.exists() || path.is_symlink() {
                // 检查符号链接
                if let Err(e) = std::fs::remove_file(&path) {
                    println!("    Warning: Failed to remove {}: {}", file, e);
                }
            }
        }

        // Remove record file
        if let Err(e) = std::fs::remove_file(&file_path) {
            println!("    Warning: Failed to remove package record: {}", e);
            continue;
        }
        println!("    Done");
    }

    Ok(())
}
