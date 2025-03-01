use clap::*;

pub fn make_subcommand() -> Command {
    Command::new("untracked")
        .about("List untracked files")
        .after_help(
            r###"
List files in ~/.cbp that are not managed by any package.

The command helps you find:
* Files not installed by any package
* Files not in system directories (records/, cache/)
* Files not required by cbp itself

Ignored files:
* System generated files
  - macOS: .DS_Store, __MACOSX, .AppleDouble
  - Windows: Thumbs.db, desktop.ini
  - Linux: backup files (*~), vim swp files

Examples:
1. List untracked files:
   cbp untracked

Note: This command is useful for cleaning up your ~/.cbp directory.
"###,
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

    println!("==> Untracked files in {}:", cbp_dirs.home.display());

    // Collect all known files from installed packages
    let mut known_files = Vec::new();
    if cbp_dirs.records.exists() {
        let files = cbp::find_files(&cbp_dirs.records, Some("*.files"))?;
        for file in files {
            let content = std::fs::read_to_string(cbp_dirs.records.join(&file))?;
            known_files.extend(content.lines().map(|s| s.to_string()));
        }
    }

    // Find and display files not in the known list
    let all_files = cbp::find_files(&cbp_dirs.home, None)?;
    for file in all_files {
        if !cbp::is_cbp_file(&file)
            && !cbp::is_system_file(&file)
            && !known_files.contains(&file)
        {
            println!("  {}", file);
        }
    }

    println!();

    Ok(())
}
