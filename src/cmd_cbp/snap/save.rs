use anyhow::Context;
use clap::*;
use std::path::{Path, PathBuf};

use cbp::libs::utils::{resolve_path, to_home_path};

pub fn make_subcommand() -> Command {
    Command::new("save")
        .about("Save files/directories as a snapshot")
        .after_help(include_str!("../../../docs/help/snap_save.md"))
        .arg(
            Arg::new("paths")
                .help("Files or directories to save")
                .required(true)
                .num_args(1..)
                .value_name("PATHS"),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .help("Output file path")
                .num_args(1)
                .value_name("FILE"),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Show verbose output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("exclude")
                .long("exclude")
                .short('x')
                .help("Exclude files matching the glob pattern")
                .action(ArgAction::Append)
                .num_args(1)
                .value_name("PATTERN"),
        )
}

pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    let verbose = args.get_flag("verbose");
    let paths: Vec<String> =
        args.get_many::<String>("paths").unwrap().cloned().collect();

    let exclude_patterns: Vec<String> = args
        .get_many::<String>("exclude")
        .map(|v| v.cloned().collect())
        .unwrap_or_default();

    let home = dirs::home_dir().context("Cannot determine HOME directory")?;

    let mut source_infos: Vec<(PathBuf, String)> = Vec::new();
    for p in &paths {
        let abs = resolve_path(Path::new(p), &home)?;
        let rel = to_home_path(&abs, &home)?;
        source_infos.push((abs, rel));
    }

    let output = match args.get_one::<String>("output") {
        Some(o) => o.clone(),
        None => {
            if source_infos.len() > 1 {
                return Err(anyhow::anyhow!(
                    "Multiple source paths require -o/--output"
                ));
            }
            let basename = source_infos[0]
                .0
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "archive".to_string());
            format!("{}.snap.tar.gz", basename)
        }
    };

    // Build comment as JSON for robust parsing
    let sources: Vec<&String> = source_infos.iter().map(|(_, rel)| rel).collect();
    let comment = serde_json::json!({
        "sources": sources,
        "exclude": exclude_patterns,
    })
    .to_string();

    let tar_file = std::fs::File::create(&output)?;
    let gz = flate2::GzBuilder::new()
        .comment(comment.as_bytes())
        .write(tar_file, flate2::Compression::default());
    let mut archive = tar::Builder::new(gz);

    let mut file_count = 0u64;

    for (abs, _rel) in &source_infos {
        if abs.is_file() {
            let name = abs.file_name().unwrap();
            if is_excluded(Path::new(name), &exclude_patterns) {
                if verbose {
                    println!("Skipped: {}", name.to_string_lossy());
                }
                continue;
            }
            archive.append_path_with_name(abs, name)?;
            file_count += 1;
            if verbose {
                println!("Added: {}", name.to_string_lossy());
            }
        } else if abs.is_dir() {
            let base_name = abs.file_name().unwrap();
            for entry in walkdir::WalkDir::new(abs) {
                let entry = entry?;
                if !entry.file_type().is_file() {
                    continue;
                }
                let file_path = entry.path();
                let archive_path =
                    Path::new(base_name).join(file_path.strip_prefix(abs)?);
                if is_excluded(&archive_path, &exclude_patterns) {
                    if verbose {
                        println!("Skipped: {}", archive_path.display());
                    }
                    continue;
                }
                archive.append_path_with_name(file_path, &archive_path)?;
                file_count += 1;
                if verbose {
                    println!("Added: {}", archive_path.display());
                }
            }
        } else {
            return Err(anyhow::anyhow!("Path not found: {}", abs.display()));
        }
    }

    archive.finish()?;
    println!("==> Snapshot created: {}", output);
    println!("==> Files: {}", file_count);
    println!("==> Source paths: {}", comment);

    Ok(())
}

fn is_excluded(path: &Path, patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    patterns.iter().any(|p| {
        glob::Pattern::new(p)
            .map(|pat| pat.matches(&path_str))
            .unwrap_or(false)
    })
}
