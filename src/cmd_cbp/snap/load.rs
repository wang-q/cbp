use anyhow::Context;
use clap::*;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn make_subcommand() -> Command {
    Command::new("load")
        .about("Restore files from a snapshot")
        .arg(
            Arg::new("archive")
                .help("Snapshot archive to restore")
                .required(true)
                .num_args(1)
                .value_name("ARCHIVE"),
        )
        .arg(
            Arg::new("target")
                .long("target")
                .short('t')
                .help("Target root directory (default: $HOME)")
                .num_args(1)
                .value_name("DIR"),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Show verbose output")
                .action(ArgAction::SetTrue),
        )
}

pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    let verbose = args.get_flag("verbose");
    let archive_path = args.get_one::<String>("archive").unwrap();
    let archive_path = Path::new(archive_path);

    if !archive_path.exists() {
        return Err(anyhow::anyhow!(
            "Archive not found: {}",
            archive_path.display()
        ));
    }

    let home = dirs::home_dir().context("Cannot determine HOME directory")?;
    let target = match args.get_one::<String>("target") {
        Some(t) => PathBuf::from(t),
        None => home.clone(),
    };

    let comment = read_comment(archive_path)?;
    if comment.is_empty() {
        return Err(anyhow::anyhow!(
            "Snapshot has no source path information in gzip comment"
        ));
    }

    let source_paths: Vec<String> = comment
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    if verbose {
        println!("==> Restoring snapshot: {}", archive_path.display());
        println!("==> Target: {}", target.display());
        println!("==> Sources: {:?}", source_paths);
    }

    let file = std::fs::File::open(archive_path)?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let entry_path = entry.path()?.to_path_buf();

        match find_target(&entry_path, &source_paths, &home, &target) {
            Some(target_path) => {
                if verbose {
                    println!(
                        "Extracting: {} -> {}",
                        entry_path.display(),
                        target_path.display()
                    );
                }
                if let Some(parent) = target_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                entry.unpack(&target_path)?;
            }
            None => {
                eprintln!(
                    "Warning: Cannot determine target for archive entry: {}",
                    entry_path.display()
                );
            }
        }
    }

    println!("==> Snapshot restored to: {}", target.display());
    Ok(())
}

fn read_comment(path: &Path) -> anyhow::Result<String> {
    let file = std::fs::File::open(path)?;
    let mut decoder = flate2::read::GzDecoder::new(file);
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf)?;
    let header = decoder.header();
    Ok(header
        .and_then(|h| h.comment())
        .map(|c| String::from_utf8_lossy(c).to_string())
        .unwrap_or_default())
}

fn find_target(
    archive_entry: &Path,
    source_paths: &[String],
    home: &Path,
    target: &Path,
) -> Option<PathBuf> {
    for source in source_paths {
        let resolved = resolve_home_path(source, home).ok()?;
        let source_basename = resolved.file_name()?.to_string_lossy().to_string();
        let source_dir = resolved
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_default();

        let entry_str = archive_entry.to_string_lossy();
        let entry_first = entry_str
            .split(std::path::MAIN_SEPARATOR)
            .next()
            .unwrap_or("");

        if entry_first == source_basename {
            let rel_to_source_dir = source_dir.strip_prefix(home).unwrap_or(&source_dir);
            let entry_after_basename = if entry_str == source_basename {
                PathBuf::new()
            } else {
                let prefix = format!("{}{}", source_basename, std::path::MAIN_SEPARATOR);
                if entry_str.starts_with(&prefix) {
                    PathBuf::from(&entry_str[prefix.len()..])
                } else {
                    continue;
                }
            };
            let target_path = target.join(rel_to_source_dir).join(entry_after_basename);
            return Some(target_path);
        }
    }
    None
}

fn resolve_home_path(path: &str, home: &Path) -> anyhow::Result<PathBuf> {
    if path == "~" {
        return Ok(home.to_path_buf());
    }
    let separator = if path.contains('/') { '/' } else { '\\' };
    if let Some(rest) = path.strip_prefix(&format!("~{}", separator)) {
        Ok(home.join(rest))
    } else if let Some(rest) = path.strip_prefix("~/") {
        Ok(home.join(rest))
    } else if let Some(rest) = path.strip_prefix("~\\") {
        Ok(home.join(rest))
    } else {
        Ok(PathBuf::from(path))
    }
}
