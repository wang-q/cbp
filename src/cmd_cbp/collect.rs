use clap::{Arg, ArgAction, Command};
use cmd_lib::*;
use std::io::Read;

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
            Arg::new("copy")
                .long("copy")
                .help("Create file aliases (e.g., --copy libzlib.a=libz.a)")
                .action(ArgAction::Append)
                .num_args(1),
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

    // Parse copy aliases
    let copy_map: std::collections::HashMap<String, Vec<String>> = matches
        .get_many::<String>("copy")
        .map(|copies| {
            let mut map = std::collections::HashMap::new();
            for copy_pair in copies {
                let parts: Vec<&str> = copy_pair.split('=').collect();
                if parts.len() == 2 {
                    map.entry(parts[0].to_string())
                        .or_insert_with(Vec::new)
                        .push(parts[1].to_string());
                }
            }
            map
        })
        .unwrap_or_default();

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

            // Check if it's a Windows executable in tools or bin directory
            if (parts[0] == "tools" || parts[0] == "bin") && is_windows_executable(&src_path)? {
                let dest_exe = dest_path.with_extension("exe");
                if dest_path != dest_exe {
                    std::fs::rename(&dest_path, &dest_exe)?;
                }
            }

            #[cfg(unix)]
            if parts[0] == "tools" {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(
                    &dest_path,
                    std::fs::Permissions::from_mode(0o755),
                )?;
            }

            // Handle file aliases
            if let Some(file_name) = dest_path.file_name().and_then(|n| n.to_str()) {
                if let Some(aliases) = copy_map.get(file_name) {
                    if let Some(parent) = dest_path.parent() {
                        for alias in aliases {
                            std::fs::copy(&dest_path, parent.join(alias))?;
                        }
                    }
                }
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

fn is_windows_executable(path: &std::path::Path) -> std::io::Result<bool> {
    let mut file = std::fs::File::open(path)?;
    let mut buffer = [0u8; 2];

    if file.read_exact(&mut buffer).is_ok() {
        // DOS MZ header magic number
        Ok(buffer == [0x4D, 0x5A])
    } else {
        Ok(false)
    }
}
