use anyhow::Result;
use clap::{ArgMatches, Command};
use std::fs;

pub fn make_subcommand() -> Command {
    Command::new("init")
        .about("Initialize cbp environment")
}

pub fn execute(_matches: &ArgMatches) -> Result<()> {
    // Get current executable path
    let current_exe = std::env::current_exe()?;
    
    // Create cbp directories
    let cbp_dirs = cbp::CbpDirs::new()?;
    
    // Copy executable to bin directory
    let target_path = cbp_dirs.bin.join("cbp");
    if current_exe != target_path {
        fs::copy(&current_exe, &target_path)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&target_path, fs::Permissions::from_mode(0o755))?;
        }
    }

    // Update PATH in shell config files
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let shell_configs = vec![".bashrc", ".bash_profile", ".zshrc"];
    
    for config in shell_configs {
        let config_path = home.join(config);
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            if !content.contains("\n# .cbp\n") {
                let mut file = fs::OpenOptions::new()
                    .append(true)
                    .open(&config_path)?;
                use std::io::Write;
                writeln!(file, "\n# .cbp")?;
                writeln!(file, "export PATH=\"$HOME/.cbp/bin:$PATH\"")?;
            }
        }
    }

    println!("cbp initialization completed!");
    println!("To make the environment variables take effect, run:");
    println!("    source ~/.bashrc  # or restart your terminal");
    println!("To verify installation:");
    println!("    cbp help");

    Ok(())
}
