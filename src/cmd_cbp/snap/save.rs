use anyhow::Context;
use clap::*;
use std::path::{Path, PathBuf};

pub fn make_subcommand() -> Command {
    Command::new("save")
        .about("Save files/directories as a snapshot")
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
}

pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    let verbose = args.get_flag("verbose");
    let paths: Vec<String> =
        args.get_many::<String>("paths").unwrap().cloned().collect();

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

    let comment = source_infos
        .iter()
        .map(|(_, rel)| rel.clone())
        .collect::<Vec<_>>()
        .join(" ");

    let tar_file = std::fs::File::create(&output)?;
    let gz = flate2::GzBuilder::new()
        .comment(comment.as_bytes())
        .write(tar_file, flate2::Compression::default());
    let mut archive = tar::Builder::new(gz);

    for (abs, _rel) in &source_infos {
        if abs.is_file() {
            let name = abs.file_name().unwrap();
            archive.append_path_with_name(abs, name)?;
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
                archive.append_path_with_name(file_path, &archive_path)?;
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
    println!("==> Source paths: {}", comment);

    Ok(())
}

fn resolve_path(path: &Path, home: &Path) -> anyhow::Result<PathBuf> {
    let path_str = path.to_string_lossy();
    let expanded = if path_str == "~" {
        home.to_path_buf()
    } else if path_str.starts_with("~/") || path_str.starts_with("~\\") {
        home.join(&path_str[2..])
    } else {
        path.to_path_buf()
    };

    if expanded.exists() {
        Ok(expanded)
    } else {
        dunce::canonicalize(&expanded)
            .with_context(|| format!("Path not found: {}", expanded.display()))
    }
}

fn to_home_path(abs: &Path, home: &Path) -> anyhow::Result<String> {
    let abs = dunce::canonicalize(abs).unwrap_or_else(|_| abs.to_path_buf());
    let home = dunce::canonicalize(home).unwrap_or_else(|_| home.to_path_buf());

    if let Ok(rel) = abs.strip_prefix(&home) {
        return Ok(format!("~/{}", rel.display()));
    }

    let mut ancestor = home.as_path();
    let mut ups = String::new();
    loop {
        if let Ok(rel) = abs.strip_prefix(ancestor) {
            return Ok(format!("~/{}/{}", ups, rel.display()));
        }
        match ancestor.parent() {
            Some(p) => {
                ancestor = p;
                ups.push_str("../");
            }
            None => return Ok(abs.to_string_lossy().to_string()),
        }
    }
}
