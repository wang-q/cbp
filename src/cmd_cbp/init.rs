use std::fs;
use std::path::PathBuf;

pub fn make_subcommand() -> clap::Command {
    clap::Command::new("init")
        .about("Initialize cbp environment")
        .after_help(
            r###"
Initialize CBP environment and configure shell settings.

Operations:
* Create ~/.cbp directory structure
* Install cbp executable
* Updates $PATH on Bash, Zsh or Windows

Configuration:
* Default: Uses ~/.cbp for everything
* Custom: Separates config and packages
  - Config stays in ~/.cbp
  - Packages go to specified directory

Examples:
* Default installation
  cbp init

* Custom package directory
  cbp init /opt/cbp

"###,
        )
        .arg(
            clap::Arg::new("home")
                .help("Custom home directory")
                .value_name("DIR")
                .index(1),
        )
        .arg(
            clap::Arg::new("dev")
                .long("dev")
                .help("Install development tools")
                .action(clap::ArgAction::SetTrue),
        )
}

pub fn execute(matches: &clap::ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Args
    //----------------------------
    // Get current executable path and resolve symlinks
    let current_exe = dunce::canonicalize(std::env::current_exe()?)?;

    // Create cbp directories
    let cbp_dirs = if let Some(home_dir) = matches.get_one::<String>("home") {
        let home = cbp::to_absolute_path(home_dir)?;
        cbp::CbpDirs::from(home)?
    } else {
        cbp::CbpDirs::new()?
    };

    //----------------------------
    // Process
    //----------------------------
    // Install development tools if --dev is specified
    if matches.get_flag("dev") {
        create_compiler_shims(&cbp_dirs.bin)?;
        create_triplet_files(&cbp_dirs.home)?;
    }

    // Copy executable to bin directory
    #[cfg(windows)]
    let target_path = cbp_dirs.bin.join("cbp.exe");
    #[cfg(not(windows))]
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
    #[cfg(unix)]
    {
        let shell_rcs = vec![".bashrc", ".bash_profile", ".zshrc"];
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        for rc in shell_rcs {
            let rc_path = home.join(rc);
            if rc_path.exists() {
                update_shell_rc(&rc_path, &cbp_dirs.bin)?;
            }
        }
    }

    #[cfg(windows)]
    {
        update_windows_path(&cbp_dirs.bin)?;
    }

    println!("cbp initialization completed!");
    println!("cbp home directory: {}", cbp_dirs.home.display());

    #[cfg(unix)]
    {
        println!("\nTo make the environment variables take effect, run:");
        println!("    source ~/.bashrc  # or restart your terminal");
    }

    #[cfg(windows)]
    {
        println!("\nTo make the environment variables take effect:");
        println!("    Please restart your terminal or log out and log back in");
    }

    println!("To verify installation:");
    println!("    cbp help");

    Ok(())
}

#[cfg(unix)]
// Generate PATH configurations
fn generate_path_configs(dir_path: &PathBuf) -> String {
    format!("export PATH=\"{}:$PATH\"", dir_path.display())
}

#[cfg(unix)]
// Update PATH in shell config files
fn update_shell_rc(rc_path: &PathBuf, bin_dir: &PathBuf) -> anyhow::Result<()> {
    let content = fs::read_to_string(rc_path)?;
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
                new_content.push(generate_path_configs(bin_dir));
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
        new_content.push(generate_path_configs(bin_dir));
        new_content.push("# .cbp end".to_string());
    }

    // Ensure file ends with newline
    if !new_content.is_empty() && !new_content.last().unwrap().is_empty() {
        new_content.push(String::new());
    }

    // Write new content
    fs::write(rc_path, new_content.join("\n") + "\n")?;
    Ok(())
}

#[cfg(windows)]
fn update_windows_path(bin_dir: &PathBuf) -> anyhow::Result<()> {
    use anyhow::Context;
    use std::process::Command;

    // Check if path already exists
    let check_output = Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "[Environment]::GetEnvironmentVariable('Path', \
                [EnvironmentVariableTarget]::User) -split ';' -contains '{}'",
                bin_dir.display()
            ),
        ])
        .output()
        .context("Failed to check PATH environment variable")?;

    if !check_output.status.success() {
        return Err(anyhow::anyhow!(
            "PowerShell command failed: {}",
            String::from_utf8_lossy(&check_output.stderr)
        ));
    }

    // If path already exists, return
    if String::from_utf8_lossy(&check_output.stdout).trim() == "True" {
        return Ok(());
    }

    // Add path to PATH
    let output = Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "$path = [Environment]::GetEnvironmentVariable('Path', \
                [EnvironmentVariableTarget]::User); \
                if ($path.EndsWith(';')) {{ $path = $path + '{}' }} \
                else {{ $path = $path + ';{}' }}; \
                [Environment]::SetEnvironmentVariable('Path', $path, \
                [EnvironmentVariableTarget]::User)",
                bin_dir.display(),
                bin_dir.display()
            ),
        ])
        .output()
        .context("Failed to update PATH environment variable")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Failed to update PATH: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

// Add new function for creating compiler shims
#[cfg(windows)]
fn create_compiler_shims(bin_dir: &PathBuf) -> anyhow::Result<()> {
    let shims = [
        ("zig-cc.cmd", "@echo off\nzig cc %*"),
        ("zig-c++.cmd", "@echo off\nzig c++ %*"),
        ("zig-ar.cmd", "@echo off\nzig ar %*"),
        ("zig-ranlib.cmd", "@echo off\nzig ranlib %*"),
    ];

    for (filename, content) in shims {
        fs::write(bin_dir.join(filename), content)?;
    }

    Ok(())
}

#[cfg(unix)]
fn create_compiler_shims(bin_dir: &PathBuf) -> anyhow::Result<()> {
    let shims = [
        ("zig-cc", "#!/bin/bash\nexec zig cc \"$@\""),
        ("zig-c++", "#!/bin/bash\nexec zig c++ \"$@\""),
        ("zig-ar", "#!/bin/bash\nexec zig ar \"$@\""),
        ("zig-ranlib", "#!/bin/bash\nexec zig ranlib \"$@\""),
    ];

    for (filename, content) in shims {
        let file_path = bin_dir.join(filename);
        fs::write(&file_path, content)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&file_path, fs::Permissions::from_mode(0o755))?;
        }
    }

    Ok(())
}

fn create_triplet_files(config_dir: &PathBuf) -> anyhow::Result<()> {
    let triplets_dir = config_dir.join("triplets");
    fs::create_dir_all(&triplets_dir)?;

    // Base toolchain files
    const ZIG_LINUX_CMAKE: &str = include_str!("../../doc/triplets/zig-linux.cmake");
    const ZIG_MACOS_CMAKE: &str = include_str!("../../doc/triplets/zig-macos.cmake");
    const ZIG_WINDOWS_CMAKE: &str = include_str!("../../doc/triplets/zig-windows.cmake");

    // Platform-specific triplet files
    const ARM64_MACOS_ZIG_CMAKE: &str =
        include_str!("../../doc/triplets/arm64-macos-zig.cmake");
    const X64_LINUX_ZIG_CMAKE: &str =
        include_str!("../../doc/triplets/x64-linux-zig.cmake");
    const X64_WINDOWS_ZIG_CMAKE: &str =
        include_str!("../../doc/triplets/x64-windows-zig.cmake");

    // Write base toolchain files
    fs::write(triplets_dir.join("zig-linux.cmake"), ZIG_LINUX_CMAKE)?;
    fs::write(triplets_dir.join("zig-macos.cmake"), ZIG_MACOS_CMAKE)?;
    fs::write(triplets_dir.join("zig-windows.cmake"), ZIG_WINDOWS_CMAKE)?;

    // Write platform-specific triplet files
    fs::write(
        triplets_dir.join("arm64-macos-zig.cmake"),
        ARM64_MACOS_ZIG_CMAKE,
    )?;
    fs::write(
        triplets_dir.join("x64-linux-zig.cmake"),
        X64_LINUX_ZIG_CMAKE,
    )?;
    fs::write(
        triplets_dir.join("x64-windows-zig.cmake"),
        X64_WINDOWS_ZIG_CMAKE,
    )?;

    Ok(())
}
