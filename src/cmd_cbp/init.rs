use anyhow::Result;
use clap::{ArgMatches, Command};
use std::fs;
use std::path::PathBuf;

pub fn make_subcommand() -> Command {
    Command::new("init")
        .about("Initialize cbp environment")
        .arg(
            clap::Arg::new("dir")
                .long("dir")
                .help("Custom installation directory")
                .value_name("DIR"),
        )
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    //----------------------------
    // Args
    //----------------------------
    // Extract custom directory from command line arguments
    let custom_dir_path = if let Some(custom_dir) = matches.get_one::<String>("dir") {
        Some(cbp::to_absolute_path(custom_dir)?)
    } else {
        None
    };

    // Get current executable path
    let current_exe = std::env::current_exe()?;

    // Get home directory
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;

    //----------------------------
    // Process
    //----------------------------
    // Create .cbp directory for config
    let cbp_config_dir = home.join(".cbp");
    fs::create_dir_all(&cbp_config_dir)?;

    // Write config file with default or custom directory
    let config_content = if let Some(ref path) = custom_dir_path {
        format!(
            r#"# CBP configuration file
home = "{}"
"#,
            path.display()
        )
    } else {
        format!(
            r#"# CBP configuration file
# Uncomment and modify to customize installation directory
# home = "/path/to/custom/dir"
"#
        )
    };

    fs::write(cbp_config_dir.join("config.toml"), config_content)?;

    // Create cbp directories
    let cbp_dirs = cbp::CbpDirs::new()?;

    // Create bin directory in config location
    let bin_dir = cbp_config_dir.join("bin");
    fs::create_dir_all(&bin_dir)?;

    // Copy executable to bin directory
    let target_path = bin_dir.join("cbp");
    if current_exe != target_path {
        fs::copy(&current_exe, &target_path)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&target_path, fs::Permissions::from_mode(0o755))?;
        }
    }

    // Update PATH in shell config files
    let shell_configs = vec![".bashrc", ".bash_profile", ".zshrc"];

    for config in shell_configs {
        let config_path = home.join(config);
        if config_path.exists() {
            update_shell_config(&config_path, custom_dir_path.as_ref())?;
        }
    }

    println!("cbp initialization completed!");
    println!("Configuration and executable: {}", cbp_config_dir.display());
    println!(
        "Package installation directory: {}",
        cbp_dirs.home.display()
    );
    println!("\nTo make the environment variables take effect, run:");
    println!("    source ~/.bashrc  # or restart your terminal");
    println!("To verify installation:");
    println!("    cbp help");

    Ok(())
}

// Generate PATH configurations
fn generate_path_configs(custom_dir_path: Option<&PathBuf>) -> Vec<String> {
    let mut configs = vec!["export PATH=\"$HOME/.cbp/bin:$PATH\"".to_string()];
    if let Some(dir_path) = custom_dir_path {
        configs.push(format!("export PATH=\"{}/bin:$PATH\"", dir_path.display()));
    }
    configs
}

// Update PATH in shell config files
fn update_shell_config(
    config_path: &PathBuf,
    custom_dir_path: Option<&PathBuf>,
) -> anyhow::Result<()> {
    let content = fs::read_to_string(config_path)?;
    let mut new_content = Vec::new();
    let mut in_cbp_section = false;
    let mut has_cbp_section = false;

    // Process existing content
    for line in content.lines() {
        match line.trim() {
            "# .cbp start" => {
                has_cbp_section = true;
                in_cbp_section = true;
                new_content.push(line.to_string());
                new_content.extend(generate_path_configs(custom_dir_path));
            }
            "# .cbp end" => {
                in_cbp_section = false;
                new_content.push(line.to_string());
            }
            _ if !in_cbp_section => new_content.push(line.to_string()),
            _ => {}
        }
    }

    // Add new config block if not exists
    if !has_cbp_section {
        if !new_content.is_empty() && !new_content.last().unwrap().is_empty() {
            new_content.push(String::new());
        }
        new_content.push("# .cbp start".to_string());
        new_content.extend(generate_path_configs(custom_dir_path));
        new_content.push("# .cbp end".to_string());
    }

    // Ensure file ends with newline
    if !new_content.is_empty() && !new_content.last().unwrap().is_empty() {
        new_content.push(String::new());
    }

    // Write new content
    fs::write(config_path, new_content.join("\n") + "\n")?;
    Ok(())
}
