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
        .subcommand(cmd_cbp::upload::make_subcommand())
        .subcommand(cmd_cbp::collect::make_subcommand())
        .after_help(
            r###"
Package Manager Features:
    * Pre-built binaries without dependencies
    * GitHub release integration
    * Local package support
    * Customizable installation paths

Directory Structure:
    ~/.cbp/
    ├── bin/      - Executable files
    ├── cache/    - Downloaded packages
    ├── records/  - Package file lists
    └── include/, lib/, share/ - Installed files

Common Commands:
1. Initial Setup:
   cbp init                    # default setup
   cbp init /opt/cbp           # custom location

2. Package Installation:
   cbp install zlib            # from GitHub
   cbp local zlib              # from local files

3. Package Management:
   cbp list                    # list installed packages
   cbp list zlib               # show package contents
   cbp remove zlib             # remove package

4. Package Discovery:
   cbp info zlib               # package information
   cbp avail                   # list all packages
   cbp avail macos             # platform specific

5. Development Tools:
   cbp check                   # find unmanaged files
   cbp tar -o pkg.tar.gz src/  # create package
   cbp prefix                  # show install paths

6. Documentation:
   cbp kb readme               # view documentation

"###,
        );

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("avail", sub_matches)) => cmd_cbp::avail::execute(sub_matches),
        Some(("check", sub_matches)) => cmd_cbp::check::execute(sub_matches),
        Some(("info", sub_matches)) => cmd_cbp::info::execute(sub_matches),
        Some(("init", sub_matches)) => cmd_cbp::init::execute(sub_matches),
        Some(("install", sub_matches)) => cmd_cbp::install::execute(sub_matches),
        Some(("kb", sub_matches)) => cmd_cbp::kb::execute(sub_matches),
        Some(("list", sub_matches)) => cmd_cbp::list::execute(sub_matches),
        Some(("local", sub_matches)) => cmd_cbp::local::execute(sub_matches),
        Some(("remove", sub_matches)) => cmd_cbp::remove::execute(sub_matches),
        Some(("tar", sub_matches)) => cmd_cbp::tar::execute(sub_matches),
        Some(("upload", sub_matches)) => cmd_cbp::upload::execute(sub_matches),
        Some(("prefix", sub_matches)) => cmd_cbp::prefix::execute(sub_matches),
        Some(("collect", sub_matches)) => cmd_cbp::collect::execute(sub_matches),
        _ => unreachable!(),
    }?;

    Ok(())
}
