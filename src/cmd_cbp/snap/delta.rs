use anyhow::Context;
use clap::*;
use std::io::Read;
use std::path::{Path, PathBuf};

use cbp::libs::utils::{
    delta_output_name, expand_home_path, find_matching_source, find_target_path,
    parse_comment, read_comment,
};

pub fn make_subcommand() -> Command {
    Command::new("delta")
        .about("Show files modified since snapshot")
        .after_help(include_str!("../../../docs/help/snap_delta.md"))
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

    let (source_paths, _exclude_patterns) = parse_comment(&comment);

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
        pack_modified(&modified, &comment, &home, &delta_name)?;
        println!(
            "==> Delta snapshot created: {} ({} files)",
            delta_name,
            modified.len()
        );
    } else {
        println!("{} files modified.", modified.len());
    }

    Ok(())
}

fn pack_modified(
    modified: &[(PathBuf, PathBuf)],
    full_comment: &str,
    home: &Path,
    output: &str,
) -> anyhow::Result<()> {
    let (source_paths, _) = parse_comment(full_comment);

    let tar_file = std::fs::File::create(output)?;
    let gz = flate2::GzBuilder::new()
        .comment(full_comment.as_bytes())
        .write(tar_file, flate2::Compression::default());
    let mut archive = tar::Builder::new(gz);

    for (target, display_path) in modified {
        let archive_name =
            if let Some(source) = find_matching_source(display_path, &source_paths) {
                let resolved = expand_home_path(&source, home);
                let base = resolved.file_name().unwrap().to_string_lossy().to_string();
                let rel = display_path
                    .strip_prefix(resolved.strip_prefix(home).unwrap_or(Path::new("")))
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
