use chrono::prelude::*;
use clap::*;
use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
    md5: String,
    created_at: String,
    path: String,
}

pub fn make_subcommand() -> Command {
    Command::new("upload")
        .about("Upload package files to GitHub release")
        .hide(true) // Hide this command from help messages
        .after_help(
            r###"
Upload package files to GitHub release and update checksums.

Requirements:
* GitHub CLI (gh) must be installed and authenticated
* Set HTTPS_PROXY if needed (e.g., export HTTPS_PROXY=http://127.0.0.1:7890)

The command will:
* Calculate MD5 checksums
* Upload files to GitHub release
* Update release notes with checksums and timestamps

Examples:
1. Upload single file:
   cbp upload binaries/zlib.macos.tar.gz

2. Upload multiple files:
   cbp upload binaries/*.tar.gz
"###,
        )
        .arg(
            Arg::new("files")
                .help("Files to upload")
                .required(true)
                .num_args(1..)
                .value_name("FILES"),
        )
}

pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    let files: Vec<_> = args.get_many::<String>("files").unwrap().collect();

    let mut release_notes = include_str!("../../doc/release.md").to_string();
    release_notes = release_notes
        .lines()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("\n");

    // Download existing package information
    let mut packages: Vec<Package> = {
        let output = std::process::Command::new("gh")
            .args([
                "release",
                "download",
                "Binaries",
                "--pattern",
                "cbp-packages.json",
                "--output",
                "-",
            ])
            .output()?;

        if output.status.success() {
            serde_json::from_slice(&output.stdout)?
        } else {
            // Check if the file doesn't exist
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("no asset") {
                println!("==> No existing package information, creating new");
                Vec::new()
            } else {
                return Err(anyhow::anyhow!(
                    "Failed to download package info: {}",
                    stderr
                ));
            }
        }
    };

    // Process each file
    let mut to_upload = Vec::new(); // Track files that need to be uploaded
    for file in &files {
        let path = Path::new(file);
        let name = path.file_name().unwrap().to_string_lossy().to_string();

        println!("==> Processing {}...", name);

        // Calculate MD5 and get file info
        let (hash, created_at) = calculate_file_info(file)?;

        // Update package information
        if let Some(existing) = packages.iter_mut().find(|p| p.name == name) {
            // Check MD5, only update and upload if different
            if existing.md5 != hash {
                existing.md5 = hash;
                existing.created_at = created_at.format("%Y-%m-%d").to_string();
                existing.path = file.to_string();
                to_upload.push(file.to_string());
                println!("==> MD5 changed, will upload");
            } else {
                println!("==> MD5 unchanged, skip upload");
            }
        } else {
            // Add new package
            packages.push(Package {
                name: name.clone(),
                md5: hash,
                created_at: created_at.format("%Y-%m-%d").to_string(),
                path: file.to_string(),
            });
            to_upload.push(file.to_string());
        }
    }

    // Upload files
    if to_upload.is_empty() {
        println!("==> No files need to be uploaded");
        return Ok(());
    }

    for file in &to_upload {
        let path = Path::new(file);
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        println!("==> Uploading {}...", name);
        std::process::Command::new("gh")
            .args(["release", "upload", "Binaries", file, "--clobber"])
            .status()?;
    }

    // Sort and save package information
    packages.sort_by(|a, b| a.name.cmp(&b.name));
    let json = serde_json::to_string_pretty(&packages)?;

    // Create a temporary directory for all intermediate files
    let temp_dir = tempfile::tempdir()?;
    let pkg_file = temp_dir.path().join("cbp-packages.json");
    std::fs::write(&pkg_file, &json)?;

    // Upload package information
    std::process::Command::new("gh")
        .args([
            "release",
            "upload",
            "Binaries",
            pkg_file.to_str().unwrap(),
            "--clobber",
        ])
        .status()?;

    // Update release notes
    let pkg_tsv = format!(
        "md5\tname\n{}",
        packages
            .iter()
            .map(|p| format!("{}\t{}", p.md5, p.name))
            .collect::<Vec<String>>()
            .join("\n")
    );

    let new_notes = format!(
        "{}\n\n### MD5 Checksums\n\n```text\n{}\n```",
        release_notes, pkg_tsv
    );
    let notes_file = temp_dir.path().join("release-notes.md");
    std::fs::write(&notes_file, &new_notes)?;

    std::process::Command::new("gh")
        .args([
            "release",
            "edit",
            "Binaries",
            "--notes-file",
            notes_file.to_str().unwrap(),
        ])
        .status()?;

    println!("==> All files processed successfully");

    Ok(())
}

fn calculate_file_info(file: &str) -> anyhow::Result<(String, DateTime<Local>)> {
    let mut file_handle = std::fs::File::open(file)?;
    let metadata = std::fs::metadata(file)?;
    let mut hasher = Md5::new();
    std::io::copy(&mut file_handle, &mut hasher)?;
    let hash = format!("{:x}", hasher.finalize());
    let created_at = DateTime::<Local>::from(metadata.created()?);
    Ok((hash, created_at))
}
