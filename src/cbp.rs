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
        .subcommand(cmd_cbp::install::make_subcommand()) // 最常用命令放前面
        .subcommand(cmd_cbp::list::make_subcommand())
        .subcommand(cmd_cbp::remove::make_subcommand())
        .subcommand(cmd_cbp::avail::make_subcommand())
        .subcommand(cmd_cbp::local::make_subcommand())
        .subcommand(cmd_cbp::check::make_subcommand())
        .subcommand(cmd_cbp::kb::make_subcommand())
        .after_help(
            r###"
Package Manager Features:
    * Cross-platform support (macOS/Linux)
    * Pre-built static binaries
    * GitHub release integration
    * Local package support
    * Package tracking

Directory Structure:
    ~/.cbp/
    ├── bin/      - Executable files
    ├── cache/    - Downloaded packages
    ├── records/  - Package file lists
    └── include/, lib/, share/ - Installed files

Common Commands:
1. Package Installation:
   cbp install zlib                                   # from GitHub
   cbp install --proxy socks5://127.0.0.1:7890 zlib   # with proxy
   cbp local zlib                                     # from local files

2. Package Management:
   cbp list                                           # list all packages
   cbp list zlib                                      # show package contents
   cbp remove zlib                                    # remove package

3. Package Discovery:
   cbp avail                                          # list available packages
   cbp check                                          # find unmanaged files

4. Documentation:
   cbp kb readme                                      # view documentation
"###,
        );

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("avail", sub_matches)) => cmd_cbp::avail::execute(sub_matches),
        Some(("install", sub_matches)) => cmd_cbp::install::execute(sub_matches), // 新增
        Some(("kb", sub_matches)) => cmd_cbp::kb::execute(sub_matches),
        Some(("list", sub_matches)) => cmd_cbp::list::execute(sub_matches),
        Some(("local", sub_matches)) => cmd_cbp::local::execute(sub_matches),
        Some(("remove", sub_matches)) => cmd_cbp::remove::execute(sub_matches),
        Some(("check", sub_matches)) => cmd_cbp::check::execute(sub_matches),
        _ => unreachable!(),
    }?;

    Ok(())
}
