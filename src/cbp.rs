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
        .after_help(
            r###"
Subcommand groups:

"###,
        );

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("kb", sub_matches)) => cmd_cbp::kb::execute(sub_matches),
        Some(("list", sub_matches)) => cmd_cbp::list::execute(sub_matches),
        _ => unreachable!(),
    }?;

    Ok(())
}
