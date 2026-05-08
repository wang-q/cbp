use clap::*;

pub mod delta;
pub mod list;
pub mod load;
pub mod save;

/// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("snap")
        .about("Manage file snapshots in HOME")
        .after_help(
            r###"
Manage file snapshots for backup and restore.

Snapshots store the original file paths in the gzip comment,
allowing files to be restored to their correct locations.

Behavior:
  - Snapshots are stored as .snap.tar.gz files
  - Source paths are saved relative to HOME when possible
  - Files outside HOME are stored with absolute paths
  - Delta snapshots capture only modified files

Examples:
1. Save a single file:
   cbp snap save ~/.bashrc
   # Creates: .bashrc.snap.tar.gz

2. Save multiple files to specific archive:
   cbp snap save ~/.bashrc ~/.vimrc -o dotfiles.snap.tar.gz

3. Save a directory:
   cbp snap save ~/.config/nvim -o nvim.snap.tar.gz

4. List snapshot contents:
   cbp snap list dotfiles.snap.tar.gz

5. Restore snapshot to HOME:
   cbp snap load dotfiles.snap.tar.gz

6. Restore to different directory:
   cbp snap load dotfiles.snap.tar.gz -t /tmp/restore

7. Check what files have changed:
   cbp snap delta dotfiles.snap.tar.gz

8. Pack modified files into delta snapshot:
   cbp snap delta dotfiles.snap.tar.gz -p
"###,
        )
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
