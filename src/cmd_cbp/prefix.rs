use anyhow::Result;
use clap::*;

pub fn make_subcommand() -> Command {
    Command::new("prefix")
        .about("Display CBP installation directories")
        .after_help(
            r###"
Display CBP directory paths.

Usage:
  cbp prefix         # Show home directory (~/.cbp)
  cbp prefix bin     # Show binary directory
  cbp prefix lib     # Show library directory
  cbp prefix include # Show include directory
  cbp prefix exe     # Show cbp executable path
"###,
        )
        .arg(
            Arg::new("directory")
                .help("Directory to display")
                .value_parser([
                    "bin", "cache", "records", "config", "include", "lib", "exe",
                ])
                .num_args(0..=1),
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

pub fn execute(matches: &ArgMatches) -> Result<()> {
    let cbp_dirs = if matches.contains_id("dir") {
        let home = std::path::Path::new(matches.get_one::<String>("dir").unwrap())
            .to_path_buf();
        cbp::CbpDirs::from(home)?
    } else {
        cbp::CbpDirs::new()?
    };

    match matches.get_one::<String>("directory").map(|s| s.as_str()) {
        Some("bin") => println!("{}", cbp_dirs.bin.display()),
        Some("cache") => println!("{}", cbp_dirs.cache.display()),
        Some("records") => println!("{}", cbp_dirs.records.display()),
        Some("config") => println!("{}", cbp_dirs.config.display()),
        Some("include") => println!("{}", cbp_dirs.home.join("include").display()),
        Some("lib") => println!("{}", cbp_dirs.home.join("lib").display()),
        Some("exe") => println!("{}", cbp_dirs.config.join("bin/cbp").display()),
        None => println!("{}", cbp_dirs.home.display()),
        _ => unreachable!(),
    }

    Ok(())
}
