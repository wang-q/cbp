use clap::*;

pub fn make_subcommand() -> Command {
    Command::new("info")
        .about("Display package information")
        .after_help(
            r###"
Get detailed package information from GitHub repository.

Information is sourced from JSON files in the packages/ directory.

Examples:
* View package information
  cbp info newick-utils     # Show specific package info
  cbp info bwa --json      # Output in JSON format

* Network proxy support
  # Priority (high to low):
  # 1. --proxy argument
  cbp info newick-utils --proxy socks5://127.0.0.1:7890
  # 2. Environment variables:
  #    ALL_PROXY
  #    HTTP_PROXY
  #    all_proxy
  #    http_proxy
"###,
        )
        .arg(
            Arg::new("package")
                .help("Package name")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .help("Output in JSON format")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("proxy")
                .long("proxy")
                .help("Proxy server URL (e.g., socks5://127.0.0.1:7890)")
                .num_args(1)
                .value_name("URL"),
        )
}

pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    let package = args.get_one::<String>("package").unwrap();
    let is_json = args.get_flag("json");
    let opt_proxy_url = args.get_one::<String>("proxy");

    // Set up HTTP agent
    let agent = cbp::create_http_agent(opt_proxy_url)?;

    // Build JSON file URL
    let raw_url = std::env::var("GITHUB_RAW_URL")
        .unwrap_or_else(|_| "https://raw.githubusercontent.com".to_string());
    let json_url = format!(
        "{}/wang-q/cbp/master/packages/{}.json",
        raw_url,
        package
    );

    // Get JSON data
    let resp = agent
        .get(&json_url)
        .set("user-agent", "cbp")
        .call()?
        .into_string()?;

    let info: serde_json::Value = serde_json::from_str(&resp)?;

    if is_json {
        // Output formatted JSON
        println!("{}", serde_json::to_string_pretty(&info)?);
    } else {
        // Format output
        println!("==> Package info: {}", package);
        println!("Name: {}", info["name"].as_str().unwrap_or("Unknown"));
        println!("Version: {}", info["version"].as_str().unwrap_or("Unknown"));
        println!("Description: {}", info["description"].as_str().unwrap_or("None"));
        println!("Homepage: {}", info["homepage"].as_str().unwrap_or("None"));
        println!("License: {}", info["license"].as_str().unwrap_or("Unknown"));
        
        if let Some(deps) = info["dependencies"].as_array() {
            println!("\nDependencies:");
            for dep in deps {
                println!("  - {}", dep.as_str().unwrap_or("Unknown"));
            }
        }
    }

    Ok(())
}
