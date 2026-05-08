use anyhow::Context;
use clap::*;
use std::path::{Path, PathBuf};
use tracing::warn;

use cbp::libs::utils::{find_target_path, read_comment};

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
    let custom_target = args.get_one::<String>("target");
    let target = match custom_target {
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
        println!("==> Source paths: {}", source_paths.join(", "));
    }

    let file = std::fs::File::open(archive_path)?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let entry_path = entry.path()?.to_path_buf();

        if custom_target.is_some() {
            let target_path = target.join(&entry_path);
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
        } else {
            match find_target_path(&entry_path, &source_paths, &home) {
                Some(source_target) => {
                    let target_path = target.join(
                        source_target.strip_prefix(&home).unwrap_or(&source_target),
                    );
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
                    warn!(
                        "Cannot determine target for archive entry: {}",
                        entry_path.display()
                    );
                }
            }
        }
    }

    println!("==> Snapshot restored to: {}", target.display());
    Ok(())
}
