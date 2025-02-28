use clap::*;
use flate2::read::GzDecoder;
use std::fs;
use tar::Archive;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("kb")
        .about("Prints docs (knowledge bases)")
        .after_help(
            r###"
* common - describe scripts/common.sh

"###,
        )
        .arg(
            Arg::new("infile")
                .help("Sets the input file to use")
                .num_args(1)
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("outfile")
                .short('o')
                .long("outfile")
                .num_args(1)
                .default_value("stdout")
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    let outfile = args.get_one::<String>("outfile").unwrap();

    static FILE_MD_COMMON: &str = include_str!("../../doc/common.md");

    match args.get_one::<String>("infile").unwrap().as_ref() {
        "common" => {
            let mut writer = intspan::writer(outfile);
            writer.write_all(FILE_MD_COMMON.as_ref())?;
        }
        _ => unreachable!(),
    };

    Ok(())
}
