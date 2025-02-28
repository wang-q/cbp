extern crate clap;
use clap::*;

mod cmd_bp;

fn main() -> anyhow::Result<()> {
    let app = Command::new("nwr")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`bp` is a Binary Package manager")
        .propagate_version(true)
        .arg_required_else_help(true)
        .color(ColorChoice::Auto)
        .subcommand(cmd_bp::kb::make_subcommand())
        .after_help(
            r###"
Subcommand groups:

"###,
        );

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("kb", sub_matches)) => cmd_bp::kb::execute(sub_matches),
        _ => unreachable!(),
    }?;

    Ok(())
}
