use anyhow::Context;
use clap::*;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn make_subcommand() -> Command {
    Command::new("delta")
        .about("Show files modified since snapshot")
        .arg(
            Arg::new("archive")
                .help("Snapshot archive to compare")
                .required(true)
                .num_args(1)
                .value_name("ARCHIVE"),
        )
        .arg(
            Arg::new("pack")
                .long("pack")
                .short('p')
                .help("Pack modified files into a delta snapshot")
                .action(ArgAction::SetTrue),
        )
}

pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    let archive_path = args.get_one::<String>("archive").unwrap();
    let archive_path = Path::new(archive_path);

    if !archive_path.exists() {
        return Err(anyhow::anyhow!(
            "Archive not found: {}",
            archive_path.display()
        ));
    }

    let home = dirs::home_dir().context("Cannot determine HOME directory")?;

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

    let pack = args.get_flag("pack");

    let mut modified: Vec<(PathBuf, PathBuf)> = Vec::new();

    let file = std::fs::File::open(archive_path)?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let entry_path = entry.path()?.to_path_buf();

        let target = find_target_path(&entry_path, &source_paths, &home);
        let Some(target) = target else {
            continue;
        };

        if !target.exists() {
            continue;
        }

        let mut archive_content = Vec::new();
        entry.read_to_end(&mut archive_content)?;
        let disk_content = match std::fs::read(&target) {
            Ok(c) => c,
            Err(_) => continue,
        };

        if archive_content != disk_content {
            let display_path =
                target.strip_prefix(&home).unwrap_or(&target).to_path_buf();
            if !pack {
                println!("{}", display_path.display());
            }
            modified.push((target, display_path));
        }
    }

    if modified.is_empty() {
        println!("No files have been modified since snapshot.");
        return Ok(());
    }

    if pack {
        let delta_name = delta_output_name(archive_path);
        pack_modified(&modified, &source_paths, &home, &delta_name)?;
        println!("==> Delta snapshot created: {}", delta_name);
    } else {
        println!("{} files modified.", modified.len());
    }

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

fn find_target_path(
    archive_entry: &Path,
    source_paths: &[String],
    home: &Path,
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
            return Some(home.join(rel_to_source_dir).join(entry_after_basename));
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

fn delta_output_name(archive: &Path) -> String {
    let stem = archive
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "archive".to_string());
    let stem = stem
        .strip_suffix(".tar")
        .unwrap_or(&stem)
        .strip_suffix(".snap")
        .unwrap_or(&stem);
    format!("{}.delta.tar.gz", stem)
}

fn pack_modified(
    modified: &[(PathBuf, PathBuf)],
    source_paths: &[String],
    home: &Path,
    output: &str,
) -> anyhow::Result<()> {
    let comment = source_paths.join(" ");

    let tar_file = std::fs::File::create(output)?;
    let gz = flate2::GzBuilder::new()
        .comment(comment.as_bytes())
        .write(tar_file, flate2::Compression::default());
    let mut archive = tar::Builder::new(gz);

    for (target, display_path) in modified {
        let archive_name =
            if let Some(source) = find_matching_source(display_path, source_paths) {
                let resolved = resolve_home_path(&source, home)?;
                let base = resolved.file_name().unwrap().to_string_lossy().to_string();
                let rel = display_path
                    .strip_prefix(
                        resolved
                            .parent()
                            .unwrap_or(Path::new(""))
                            .strip_prefix(home)
                            .unwrap_or(Path::new("")),
                    )
                    .unwrap_or(display_path);
                PathBuf::from(&base).join(rel)
            } else {
                display_path.to_path_buf()
            };
        archive.append_path_with_name(target, &archive_name)?;
    }

    archive.finish()?;
    Ok(())
}

fn find_matching_source(display_path: &Path, source_paths: &[String]) -> Option<String> {
    let display = display_path.to_string_lossy().to_string();
    let display = if display.starts_with('/') || display.starts_with('\\') {
        display
    } else {
        format!("/{}", display)
    };

    for source in source_paths {
        let source_no_tilde = source.strip_prefix('~').unwrap_or(source);
        if display.starts_with(source_no_tilde) || display.contains(source_no_tilde) {
            return Some(source.clone());
        }
    }
    source_paths.first().cloned()
}
