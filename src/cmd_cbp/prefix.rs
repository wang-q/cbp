use anyhow::Result;
use clap::{ArgMatches, Command};

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
            clap::Arg::new("directory")
                .help("Directory to display")
                .value_parser([
                    "bin", "cache", "records", "config", "include", "lib", "exe",
                ])
                .num_args(0..=1),
        )
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    let dirs = cbp::CbpDirs::new()?;
    let config_dir = cbp::get_cbp_config_dir()?;

    match matches.get_one::<String>("directory").map(|s| s.as_str()) {
        Some("bin") => println!("{}", dirs.bin.display()),
        Some("cache") => println!("{}", dirs.cache.display()),
        Some("records") => println!("{}", dirs.records.display()),
        Some("config") => println!("{}", config_dir.display()),
        Some("include") => println!("{}", dirs.home.join("include").display()),
        Some("lib") => println!("{}", dirs.home.join("lib").display()),
        Some("exe") => println!("{}", config_dir.join("bin/cbp").display()),
        None => println!("{}", dirs.home.display()),
        _ => unreachable!(),
    }

    Ok(())
}
