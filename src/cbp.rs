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
        .subcommand(cmd_cbp::init::make_subcommand())
        .subcommand(cmd_cbp::install::make_subcommand())
        .subcommand(cmd_cbp::local::make_subcommand())
        .subcommand(cmd_cbp::list::make_subcommand())
        .subcommand(cmd_cbp::remove::make_subcommand())
        .subcommand(cmd_cbp::info::make_subcommand())
        .subcommand(cmd_cbp::avail::make_subcommand())
        .subcommand(cmd_cbp::check::make_subcommand())
        .subcommand(cmd_cbp::tar::make_subcommand())
        .subcommand(cmd_cbp::prefix::make_subcommand())
        .subcommand(cmd_cbp::kb::make_subcommand())
        .subcommand(cmd_cbp::build::make_subcommand())
        .subcommand(cmd_cbp::collect::make_subcommand())
        .subcommand(
            Command::new("uninstall")
                .about("Alias for remove command")
                .hide(true) // Hide from help message to avoid confusion
                .args(cmd_cbp::remove::make_subcommand().get_arguments().cloned()),
        )
        .after_help(
            r###"
Package Manager Features:
    * Linux/macOS/Windows
    * Pre-built binaries without dependencies
    * Customizable installation paths

Directory Structure:
    ~/.cbp/
    ├── bin/      - Executable files
    ├── cache/    - Downloaded packages
    ├── records/  - Package file lists
    └── include/, lib/, share/ - Installed files

Quick Start:
    cbp init                    # Initial setup
    cbp install <package>       # Install package
    cbp list                    # List installed packages
    cbp avail                   # List available packages
    cbp kb readme               # View more examples

"###,
        );

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("avail", sub_matches)) => cmd_cbp::avail::execute(sub_matches),
        Some(("build", sub_matches)) => cmd_cbp::build::execute(sub_matches),
        Some(("check", sub_matches)) => cmd_cbp::check::execute(sub_matches),
        Some(("collect", sub_matches)) => cmd_cbp::collect::execute(sub_matches),
        Some(("info", sub_matches)) => cmd_cbp::info::execute(sub_matches),
        Some(("init", sub_matches)) => cmd_cbp::init::execute(sub_matches),
        Some(("install", sub_matches)) => cmd_cbp::install::execute(sub_matches),
        Some(("kb", sub_matches)) => cmd_cbp::kb::execute(sub_matches),
        Some(("list", sub_matches)) => cmd_cbp::list::execute(sub_matches),
        Some(("local", sub_matches)) => cmd_cbp::local::execute(sub_matches),
        Some(("prefix", sub_matches)) => cmd_cbp::prefix::execute(sub_matches),
        Some(("remove", sub_matches)) => cmd_cbp::remove::execute(sub_matches),
        Some(("tar", sub_matches)) => cmd_cbp::tar::execute(sub_matches),
        Some(("uninstall", sub_matches)) => cmd_cbp::remove::execute(sub_matches), // Handle alias subcommand
        _ => unreachable!(),
    }?;

    Ok(())
}
