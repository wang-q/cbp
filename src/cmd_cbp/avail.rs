use clap::*;

/// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("avail")
        .about("List available packages")
        .after_help(
            r###"
List available packages from the release repository.

The command will:
* Query GitHub release assets
* Filter by platform (macos/linux)
* Group packages by first letter

Release page:
* https://github.com/wang-q/cbp/releases/tag/Binaries

Examples:
1. List all available packages:
   cbp avail

2. List packages for specific platform:
   cbp avail macos
   cbp avail linux
"###,
        )
        .arg(
            Arg::new("platform")
                .help("Target platform (macos/linux)")
                .num_args(0..=1)
                .index(1)
                .value_name("PLATFORM"),
        )
}

/// Execute avail command
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    // Query GitHub releases using ureq
    let resp: serde_json::Value =
        ureq::get("https://api.github.com/repos/wang-q/cbp/releases/tags/Binaries")
            .set("User-Agent", "cbp")
            .call()?
            .into_json()?;

    // Extract and filter package names
    let mut packages: Vec<String> = Vec::new();
    let pattern = if let Some(platform) = args.get_one::<String>("platform") {
        format!("\\.{}\\.tar\\.gz$", platform)
    } else {
        String::from("\\.(linux|macos)\\.tar\\.gz$")
    };

    let re = regex::Regex::new(&pattern)?;
    if let Some(assets) = resp["assets"].as_array() {
        for asset in assets {
            if let Some(name) = asset["name"].as_str() {
                if re.is_match(name) {
                    if let Some(pkg_name) = re.replace(name, "").into_owned().into() {
                        packages.push(pkg_name);
                    }
                }
            }
        }
    }

    packages.sort();
    packages.dedup();

    // Output results
    if let Some(platform) = args.get_one::<String>("platform") {
        println!("==> Available packages for {}:", platform);
    } else {
        println!("==> Available packages:");
    }
    if !packages.is_empty() {
        println!("{}", cbp::format_packages(&packages));
    }
    println!();

    Ok(())
}
