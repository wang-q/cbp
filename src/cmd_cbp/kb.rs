use clap::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("kb")
        .about("Display documentation (knowledge bases)")
        .after_help(
            r###"
Display documentation from the knowledge base.

Available documents:
* readme    - System requirements and basic usage
* common    - Common build functions in scripts/common.sh
* sources   - Source code organization and management
* binaries  - Binary package management
* developer - Development guide and contribution

Examples:
1. Display README:
   cbp kb readme

2. Save documentation to file:
   cbp kb developer -o dev-guide.md
"###,
        )
        .arg(
            Arg::new("name")
                .help("Name of the knowledge base to display")
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

    static FILE_MD_README: &str = include_str!("../../README.md");
    static FILE_MD_COMMON: &str = include_str!("../../doc/common.md");
    static FILE_MD_SOURCES: &str = include_str!("../../doc/sources.md");
    static FILE_MD_BINARIES: &str = include_str!("../../doc/binaries.md");
    static FILE_MD_DEVELOPER: &str = include_str!("../../doc/developer.md");

    match args.get_one::<String>("name").unwrap().as_ref() {
        "readme" => {
            let mut writer = cbp::writer(outfile);
            writer.write_all(FILE_MD_README.as_ref())?;
        }
        "common" => {
            let mut writer = cbp::writer(outfile);
            writer.write_all(FILE_MD_COMMON.as_ref())?;
        }
        "sources" => {
            let mut writer = cbp::writer(outfile);
            writer.write_all(FILE_MD_SOURCES.as_ref())?;
        }
        "binaries" => {
            let mut writer = cbp::writer(outfile);
            writer.write_all(FILE_MD_BINARIES.as_ref())?;
        }
        "developer" => {
            let mut writer = cbp::writer(outfile);
            writer.write_all(FILE_MD_DEVELOPER.as_ref())?;
        }
        _ => unreachable!(),
    };

    Ok(())
}
