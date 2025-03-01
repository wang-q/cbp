use clap::*;

pub fn make_subcommand() -> Command {
    Command::new("remove")
        .about("Remove installed packages")
        .after_help(
            r###"
Remove installed packages and their files.

The command will:
* Remove all files installed by the package
* Remove package record file
* Skip directories (they will be kept)
* Handle both regular files and symbolic links
* Skip non-existent files

Examples:
1. Remove a single package:
   cbp remove zlib

2. Remove multiple packages:
   cbp remove zlib bzip2

Note: This command cannot be undone.
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
        if file_path.exists() {
            println!("==> Removing {}:", package);

            // 读取文件列表
            let content = std::fs::read_to_string(&file_path)?;
            let mut removed_count = 0;

            // 处理每个文件
            for line in content.lines() {
                if line.is_empty() {
                    continue;
                }

                let file = cbp_dirs.home.join(line);
                if file.exists() {
                    if file.is_file() || file.is_symlink() {
                        std::fs::remove_file(&file)?;
                        removed_count += 1;

                        // 检查并删除对应的资源分支文件
                        if let Some(parent) = file.parent() {
                            if let Some(file_name) = file.file_name() {
                                if let Some(file_name_str) = file_name.to_str() {
                                    let resource_fork =
                                        parent.join(format!("._{}", file_name_str));
                                    if resource_fork.exists() {
                                        std::fs::remove_file(&resource_fork)?;
                                        println!(
                                            "    Removed resource fork: {}",
                                            resource_fork.display()
                                        );
                                    }
                                }
                            }
                        }
                    } else {
                        println!("    Skipping directory: {}", file.display());
                    }
                } else {
                    println!("    File not found: {}", file.display());
                }
            }

            // 删除记录文件
            std::fs::remove_file(&file_path)?;
            println!("    Removed {} files", removed_count);
        } else {
            println!("==> Package {} is not installed", package);
        }
    }

    Ok(())
}
