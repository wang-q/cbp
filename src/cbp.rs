extern crate clap;
use clap::*;

mod cmd_cbp;

fn main() -> anyhow::Result<()> {
    let app = Command::new("cbp")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`cbp` is a Cross-platform Binary Package manager")
        .propagate_version(true)
        .arg_required_else_help(true)
        .color(ColorChoice::Auto)
        .subcommand(cmd_cbp::kb::make_subcommand())
        .subcommand(cmd_cbp::list::make_subcommand())
        .subcommand(cmd_cbp::local::make_subcommand())
        .subcommand(cmd_cbp::remove::make_subcommand())
        .subcommand(cmd_cbp::untracked::make_subcommand())
        .after_help(
            r###"
Package Types:
    * Pre-built binaries (.tar.gz)
    * Platform specific (macos/linux)
    * Static linking preferred

Directory Structure:
    ~/.cbp/
    ├── bin/      - Executable files
    ├── cache/    - Downloaded packages
    ├── records/  - Package file lists
    └── include/, lib/, share/ - Installed files

Examples:
1. View documentation:
   cbp kb readme

2. Install a package:
   cbp local zlib

3. List installed packages:
   cbp list

4. Remove a package:
   cbp remove zlib

5. Find unmanaged files:
   cbp untracked
"###,
        );

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("kb", sub_matches)) => cmd_cbp::kb::execute(sub_matches),
        Some(("list", sub_matches)) => cmd_cbp::list::execute(sub_matches),
        Some(("local", sub_matches)) => cmd_cbp::local::execute(sub_matches),
        Some(("remove", sub_matches)) => cmd_cbp::remove::execute(sub_matches),
        Some(("untracked", sub_matches)) => cmd_cbp::untracked::execute(sub_matches),
        _ => unreachable!(),
    }?;

    Ok(())
}
