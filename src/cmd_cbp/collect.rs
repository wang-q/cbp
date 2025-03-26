use clap::{Arg, ArgAction, Command};
use cmd_lib::*;
use std::io::{Read, Seek, SeekFrom};

pub fn make_subcommand() -> Command {
    Command::new("collect")
        .about("Collect and package files into a tar.gz archive")
        .after_help(
            r###"
Collect and package files into a tar.gz archive.

Mode options:
* files: Default mode, collect files as-is
* list: Process a list file containing file paths
* vcpkg: Process vcpkg-style list file
* bin: Place files in bin/ directory
* font: Place files in share/fonts/ directory

Examples:
1. Process files (default mode):
   cbp collect program.exe --mode files
   # or simply
   cbp collect program.exe

2. Process list file:
   cbp collect list.txt --mode list

3. Process vcpkg list:
   cbp collect pkg.list --mode vcpkg

4. Create file aliases:
   cbp collect program.exe --copy libz.so=libz.so.1

5. Ignore specific files:
   cbp collect src/ --ignore .dll --ignore .exe

6. Specify output file:
   cbp collect program.exe -o output.tar.gz

7. Collect binaries:
   cbp collect program.exe --mode bin

8. Collect fonts:
   cbp collect font.ttf --mode font
"###,
        )
        .arg(
            Arg::new("sources")
                .help("Source files, directories or a list")
                .required(true)
                .num_args(1..)
                .index(1),
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
                .help("Ignore files matching the pattern")
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
        .arg(
            Arg::new("mode")
                .long("mode")
                .help("Processing mode")
                .value_parser(["files", "list", "vcpkg", "bin", "font"])
                .default_value("files")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("shebang")
                .long("shebang")
                .help("Fix shebang lines in script files")
                .action(ArgAction::SetTrue),
        )
}

pub fn execute(matches: &clap::ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Args
    //----------------------------
    let sources = matches.get_many::<String>("sources").unwrap();
    let mode = matches.get_one::<String>("mode").unwrap();
    let is_vcpkg = mode == "vcpkg";

    // Get first source for output name
    let first_source = sources.clone().into_iter().next().unwrap();

    // output name
    let output: String =
        matches
            .get_one::<String>("output")
            .cloned()
            .unwrap_or_else(|| {
                if mode == "vcpkg" {
                    let source_file = std::path::Path::new(first_source);
                    let stem = source_file.file_stem().unwrap().to_str().unwrap();
                    // Parse package name and platform from filename
                    let parts: Vec<&str> = stem.split('_').collect();
                    let pkg_name = parts[0];
                    let platform = parts
                        .get(2)
                        .map(|s| s.split('-').nth(1).unwrap_or(""))
                        .unwrap_or("");
                    format!("{}.{}.tar.gz", pkg_name, platform)
                } else {
                    let path = std::path::Path::new(first_source);
                    let name = path
                        .file_stem()
                        .or_else(|| path.file_name())
                        .and_then(|n| n.to_str())
                        .unwrap_or("output");
                    format!("{}.tar.gz", name)
                }
            });

    // Parse copy aliases and ignore patterns...
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

    let cbp = std::env::current_exe()?.display().to_string();

    //----------------------------
    // Operating
    //----------------------------
    // Process sources
    let mut file_list = Vec::new();
    let mut base_dir = std::env::current_dir()?;

    if mode == "files" || mode == "bin" || mode == "font" {
        // Collect files from command line arguments
        for source in sources {
            let path = std::path::Path::new(source);
            if !path.exists() {
                eprintln!("Warning: Source not found: {}", source);
                continue;
            }
            if path.is_file() {
                file_list.push(source.to_string());
            } else if path.is_dir() {
                let base = path.canonicalize()?;
                let files = cbp::find_files(&base, None)?;
                // eprintln!("files = {:#?}", files);
                for file in files {
                    // Combine base path with file path
                    let full_path = base.join(&file);
                    let rel_path = if let Ok(rel) = full_path.strip_prefix(&base_dir) {
                        rel.to_string_lossy().into_owned()
                    } else {
                        // If unable to get relative path, use original path
                        path.join(&file).to_string_lossy().into_owned()
                    };
                    file_list.push(rel_path);
                }
            }
        }
    } else {
        // Read and parse list file
        let source_path = sources.into_iter().next().unwrap();
        let source_file = std::path::Path::new(source_path);
        if !source_file.exists() {
            anyhow::bail!("Source file not found: {}", source_path);
        }
        let content = std::fs::read_to_string(source_file)?;
        file_list = content.lines().map(|s| s.to_string()).collect();

        if is_vcpkg {
            // For vcpkg list, use parent^3 as base
            base_dir = source_file
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .to_path_buf();
        } else {
            // For normal list, use parent as base
            base_dir = source_file.parent().unwrap().to_path_buf();
        }
    }

    eprintln!("base_dir = {:#?}", base_dir);
    // eprintln!("file_list = {:#?}", file_list);

    // Create temporary directory
    let temp_dir = tempfile::Builder::new().prefix("cbp-collect-").tempdir()?;

    // Collect files
    for line in &file_list {
        if should_skip_file(line, &ignore_patterns) {
            continue;
        }

        let src_path = base_dir.join(line);
        if !src_path.exists() {
            eprintln!("Warning: File not found: {}", src_path.display());
            continue;
        }

        let parts = match get_path_parts(line, is_vcpkg) {
            Some(parts) => parts,
            None => continue,
        };

        let relative_path = get_relative_path(&parts, mode, is_vcpkg);
        let dest_path = temp_dir.path().join(&relative_path);

        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        if src_path.is_file() {
            // can't handle symlinks
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
            if parts[0] == "tools" || parts[0] == "bin" {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(
                    &dest_path,
                    std::fs::Permissions::from_mode(0o755),
                )?;
            }

            // Fix shebang if needed
            if matches.get_flag("shebang") {
                cbp::fix_shebang(&dest_path)?;
            }

            process_file_aliases(&dest_path, &copy_map)?;
        }
    }

    // Create archive
    let temp_path = temp_dir.path().display().to_string();
    run_cmd!(
        ${cbp} tar ${temp_path} -o ${output}
    )?;

    Ok(())
}

fn should_skip_file(line: &str, ignore_patterns: &[String]) -> bool {
    ignore_patterns.iter().any(|pattern| line.contains(pattern))
}
fn get_path_parts(line: &str, is_vcpkg: bool) -> Option<Vec<String>> {
    let parts: Vec<String> = if is_vcpkg {
        line.split('/').skip(1).map(String::from).collect()
    } else {
        line.split('/').map(String::from).collect()
    };
    if parts.is_empty() {
        None
    } else {
        Some(parts)
    }
}
fn get_relative_path(parts: &[String], mode: &str, is_vcpkg: bool) -> String {
    if is_vcpkg && parts[0] == "tools" {
        format!("bin/{}", parts.last().unwrap())
    } else if mode == "bin" {
        format!("bin/{}", parts.last().unwrap())
    } else if mode == "font" {
        format!("share/fonts/{}", parts.last().unwrap())
    } else {
        parts.join("/")
    }
}

fn process_file_aliases(
    dest_path: &std::path::Path,
    copy_map: &std::collections::HashMap<String, Vec<String>>,
) -> anyhow::Result<()> {
    if let Some(file_name) = dest_path.file_name().and_then(|n| n.to_str()) {
        if let Some(aliases) = copy_map.get(file_name) {
            let parent = dest_path.parent().ok_or_else(|| {
                anyhow::anyhow!(
                    "Failed to get parent directory for {}",
                    dest_path.display()
                )
            })?;
            for alias in aliases {
                std::fs::copy(dest_path, parent.join(alias)).map_err(|e| {
                    anyhow::anyhow!("Failed to create alias {}: {}", alias, e)
                })?;
            }
        }
    }
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
        let pe_offset =
            u32::from_le_bytes(buffer[0x3C..0x40].try_into().unwrap()) as u64;
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
