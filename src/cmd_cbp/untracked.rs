use clap::*;

pub fn make_subcommand() -> Command {
    Command::new("untracked")
        .about("List untracked files")
        .after_help(
            r###"
List files in ~/.cbp that are not managed by any package.

Usage:
    cbp untracked

These files are:
* Not installed by any package
* Not in binaries/ or cache/ directories
* Not the cbp executable itself

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
        let home = std::path::Path::new(args.get_one::<String>("dir").unwrap()).to_path_buf();
        cbp::CbpDirs::from(home)?
    } else {
        cbp::CbpDirs::new()?
    };

    println!("==> Untracked files in {}:", cbp_dirs.home.display());

    // Collect all known files from installed packages
    let mut known_files = Vec::new();
    if cbp_dirs.binaries.exists() {
        let files = cbp::find_files(&cbp_dirs.binaries, Some("*.files"))?;
        for file in files {
            let content = std::fs::read_to_string(cbp_dirs.binaries.join(&file))?;
            known_files.extend(content.lines().map(|s| s.to_string()));
        }
    }

    // Find and display files not in the known list
    let all_files = cbp::find_files(&cbp_dirs.home, None)?;
    for file in all_files {
        // Skip system files and package managed files
        if file != "bin/cbp" && 
           !file.starts_with("binaries/") && 
           !file.starts_with("cache/") && 
           // macOS system files
           !file.ends_with(".DS_Store") &&     
           !file.contains("/__MACOSX/") &&     
           !file.ends_with(".AppleDouble") &&  
           // Windows system files
           !file.ends_with("Thumbs.db") &&     
           !file.ends_with("desktop.ini") &&   
           // Linux hidden files
           !file.ends_with("~") &&             
           !file.ends_with(".swp") &&          
           !known_files.contains(&file) {
            println!("  {}", file);
        }
    }

    println!();

    Ok(())
}