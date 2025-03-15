use clap::{Arg, ArgAction, Command};
use cmd_lib::*;
use std::io::{Read, Seek, SeekFrom};

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
            Arg::new("ignore")
                .long("ignore")
                .help("Ignore files matching the pattern (e.g., --ignore .dll)")
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

    // Parse ignore patterns
    let ignore_patterns: Vec<String> = matches
        .get_many::<String>("ignore")
        .map(|ignores| ignores.map(|s| s.to_string()).collect())
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
        // Check if path matches any ignore pattern
        if ignore_patterns.iter().any(|pattern| line.contains(pattern)) {
            continue;
        }

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
            if (parts[0] == "tools" || parts[0] == "bin")
                && is_windows_executable(&src_path)?
            {
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
    let mut buffer = [0u8; 0x40];

    if file.read_exact(&mut buffer).is_ok() {
        // Check DOS MZ signature (0x4D5A)
        // Returns false if not a PE file
        if buffer[0..2] != [0x4D, 0x5A] {
            return Ok(false);
        }

        // Get PE header offset from DOS header at 0x3C
        // and read PE header (24 bytes)
        let pe_offset = u32::from_le_bytes(buffer[0x3C..0x40].try_into().unwrap()) as u64;
        let mut pe_header = [0u8; 24];
        file.seek(SeekFrom::Start(pe_offset))?;
        if file.read_exact(&mut pe_header).is_err() {
            return Ok(false);
        }

        // Verify PE signature ("PE\0\0" = 0x50450000)
        if pe_header[0..4] != [0x50, 0x45, 0x00, 0x00] {
            return Ok(false);
        }

        // Get characteristics from PE header (offset 22)
        // IMAGE_FILE_DLL = 0x2000
        // Returns true if not a DLL
        let characteristics = u16::from_le_bytes(pe_header[22..24].try_into().unwrap());
        Ok((characteristics & 0x2000) == 0)
    } else {
        Ok(false)
    }
}
