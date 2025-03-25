use clap::*;

pub mod download;

/// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("build")
        .about("Build package commands")
        .subcommand_required(true)
        .subcommand(download::make_subcommand())
}

/// Execute pkg command
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    match args.subcommand() {
        Some(("download", sub_args)) => download::execute(sub_args),
        _ => unreachable!(
            "Exhausted list of subcommands and subcommand_required prevents `None`"
        ),
    }
}
