use clap::*;

/// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("avail")
        .about("List available packages from GitHub")
        .after_help(
            r###"
Query and list available packages from GitHub release repository.

This command queries the GitHub API to retrieve package information, supporting
platform-specific filtering for macOS, Linux, and Windows. Results are displayed
in a formatted table, grouped alphabetically for better readability.

[Release page](https://github.com/wang-q/cbp/releases/tag/Binaries)

Examples:
* Platform filtering
  cbp avail             # List all packages
  cbp avail linux       # Linux packages only
  cbp avail font        # Fonts

* Network proxy support
  # Priority (high to low):
  # 1. --proxy argument
  cbp avail --proxy socks5://127.0.0.1:7890
  # 2. Environment variables:
  #    ALL_PROXY
  #    HTTP_PROXY
  #    all_proxy
  #    http_proxy

"###,
        )
        .arg(
            Arg::new("platform")
                .help("Target platform (linux/macos/windows/font)")
                .num_args(0..=1)
                .index(1)
                .value_name("PLATFORM"),
        )
        .arg(
            Arg::new("proxy")
                .long("proxy")
                .help("Proxy server URL (e.g., socks5://127.0.0.1:7890)")
                .num_args(1)
                .value_name("URL"),
        )
}

/// Execute avail command
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Args
    //----------------------------
    let opt_platform = args.get_one::<String>("platform");
    let opt_proxy_url = args.get_one::<String>("proxy");

    // Set up HTTP agent with optional proxy
    let agent = cbp::create_http_agent(opt_proxy_url)?;

    let api_url = std::env::var("GITHUB_API_URL")
        .unwrap_or_else(|_| "https://api.github.com".to_string());

    //----------------------------
    // Processing
    //----------------------------
    // Query GitHub releases using ureq
    let resp: serde_json::Value = agent
        .get(&format!(
            "{}/repos/wang-q/cbp/releases/tags/Binaries",
            api_url
        ))
        .set("user-agent", "cbp")
        .call()?
        .into_json()?;

    // Extract and filter package names
    let pattern = if let Some(platform) = opt_platform {
        format!("\\.{}\\.tar\\.gz$", platform)
    } else {
        String::from("\\.(linux|macos|windows|font)\\.tar\\.gz$")
    };
    let re = regex::Regex::new(&pattern)?;

    let mut packages: Vec<String> = Vec::new();
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
