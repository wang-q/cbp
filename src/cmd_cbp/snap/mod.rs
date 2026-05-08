use clap::*;

pub mod delta;
pub mod list;
pub mod load;
pub mod save;

/// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("snap")
        .about("Manage file snapshots in HOME")
        .after_help(include_str!("../../../docs/help/snap.md"))
        .subcommand_required(true)
        .subcommand(save::make_subcommand())
        .subcommand(load::make_subcommand())
        .subcommand(list::make_subcommand())
        .subcommand(delta::make_subcommand())
}

/// Execute snap command
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    match args.subcommand() {
        Some(("save", sub_args)) => save::execute(sub_args),
        Some(("load", sub_args)) => load::execute(sub_args),
        Some(("list", sub_args)) => list::execute(sub_args),
        Some(("delta", sub_args)) => delta::execute(sub_args),
        _ => unreachable!(
            "Exhausted list of subcommands and subcommand_required prevents `None`"
        ),
    }
}
