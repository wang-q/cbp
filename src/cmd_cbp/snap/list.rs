use clap::*;
use std::io::Read;
use std::path::Path;

pub fn make_subcommand() -> Command {
    Command::new("list")
        .about("List contents of a snapshot")
        .arg(
            Arg::new("archive")
                .help("Snapshot archive to inspect")
                .required(true)
                .num_args(1)
                .value_name("ARCHIVE"),
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

    let comment = read_comment(archive_path)?;
    if comment.is_empty() {
        println!("No source path information in gzip comment");
    } else {
        println!("Source paths:");
        for src in comment.split(' ').filter(|s| !s.is_empty()) {
            println!("  {}", src);
        }
    }

    println!();
    println!("Archive contents:");

    let file = std::fs::File::open(archive_path)?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);

    let mut count = 0u64;
    let mut total_size = 0u64;
    for entry in archive.entries()? {
        let entry = entry?;
        let path = entry.path()?.to_path_buf();
        let size = entry.header().size()?;
        if verbose {
            println!("  {}  {}", format_size(size), path.display());
        } else {
            println!("  {}", path.display());
        }
        count += 1;
        total_size += size;
    }

    println!();
    println!("{} files, {} total", count, format_size(total_size));

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

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}K", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1}M", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1}G", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}
