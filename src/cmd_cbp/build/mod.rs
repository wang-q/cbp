use clap::*;

pub mod font;
pub mod source;

/// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("build")
        .about("Build package commands")
        .subcommand_required(true)
        .subcommand(source::make_subcommand())
        .subcommand(font::make_subcommand())
}

/// Execute pkg command
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    match args.subcommand() {
        Some(("font", sub_args)) => font::execute(sub_args),
        Some(("source", sub_args)) => source::execute(sub_args),
        _ => unreachable!(
            "Exhausted list of subcommands and subcommand_required prevents `None`"
        ),
    }
}
