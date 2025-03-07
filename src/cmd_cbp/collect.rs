use clap::{Arg, ArgAction, Command};
use cmd_lib::*;

pub fn make_subcommand() -> Command {
    Command::new("collect")
        .about("Collect and package files from vcpkg installed directory")
        .arg(
            Arg::new("list")
                .help("Path to the .list file")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output tar.gz file")
                .action(ArgAction::Set),
        )
}

pub fn execute(matches: &clap::ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Args
    //----------------------------
    let list_path = matches.get_one::<String>("list").unwrap();
    let list_file = std::path::Path::new(list_path);

    // Validate list file existence and extension
    if !list_file.exists() || list_file.extension().unwrap_or_default() != "list" {
        anyhow::bail!("Invalid list file: {}", list_path);
    }

    // output name
    let output: String =
        matches
            .get_one::<String>("output")
            .cloned()
            .unwrap_or_else(|| {
                let stem = list_file.file_stem().unwrap().to_str().unwrap();
                // Parse package name and platform from filename (e.g., "bzip2_1.0.8_x64-windows-zig")
                let parts: Vec<&str> = stem.split('_').collect();
                let pkg_name = parts[0];
                let platform = parts
                    .get(2)
                    .map(|s| s.split('-').nth(1).unwrap_or(""))
                    .unwrap_or("");
                format!("{}.{}.tar.gz", pkg_name, platform)
            });

    let cbp = std::env::current_exe()?.display().to_string();

    //----------------------------
    // Operating
    //----------------------------
    // Read and parse list file
    let content = std::fs::read_to_string(list_file)?;
    let vcpkg_installed = list_file
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    eprintln!("vcpkg_installed = {:#?}", vcpkg_installed);

    // Create temporary directory
    let temp_dir = tempfile::Builder::new().prefix("cbp-collect-").tempdir()?;

    // Collect files
    for line in content.lines() {
        let src_path = vcpkg_installed.join(line);
        if !src_path.exists() {
            eprintln!("Warning: Source file not found: {}", src_path.display());
            continue;
        }

        // Skip the first directory component (triplet name)
        let parts: Vec<&str> = line.split('/').skip(1).collect();
        if parts.is_empty() {
            continue;
        }

        // Special handling for files in tools directory
        let relative_path = if parts[0] == "tools" {
            format!("bin/{}", parts.last().unwrap())
        } else {
            parts.join("/")
        };

        let dest_path = temp_dir.path().join(&relative_path);
        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        if src_path.is_file() {
            std::fs::copy(&src_path, &dest_path)?;
            #[cfg(unix)]
            if parts[0] == "tools" {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&dest_path, std::fs::Permissions::from_mode(0o755))?;
            }
        }
    }

    // Create archive
    let temp_path = temp_dir.path().display().to_string();
    run_cmd!(
        ${cbp} tar ${temp_path} -o ${output}
    )?;

    Ok(())
}
