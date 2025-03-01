use clap::*;
use std::path::Path;

pub fn make_subcommand() -> Command {
    Command::new("tar")
        .about("Create compressed archive")
        .after_help(
            r###"
Create a compressed archive from a directory.

The command will:
* Filter out system files
* Preserve relative paths
* Clean up documentation (optional)

Examples:
1. Default directory:
   cbp tar

2. Custom directory:
   cbp tar path/to/dir

3. Custom output:
   cbp tar -o output.tar.gz

4. Clean up docs:
   cbp tar --cleanup
"###,
        )
        .arg(
            Arg::new("dir")
                .help("Source directory")
                .num_args(1)
                .value_name("DIR")
                .index(1)
                .default_value("collect"),
        )
        .arg(
            Arg::new("cleanup")
                .long("cleanup")
                .help("Remove documentation directories")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("outfile")
                .long("outfile")
                .short('o')
                .help("Output file path")
                .num_args(1)
                .value_name("FILE"),
        )
}

pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Args
    //----------------------------
    let collect_dir = Path::new(args.get_one::<String>("dir").unwrap());
    if !collect_dir.exists() {
        return Err(anyhow::anyhow!(
            "Directory not found: {}",
            collect_dir.display()
        ));
    }

    let tar_name = args
        .get_one::<String>("outfile")
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            format!(
                "{}.tar.gz",
                collect_dir
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "archive".to_string())
            )
        });

    let cleanup = args.get_flag("cleanup");
    let doc_dirs = ["share/info/", "share/man/", "share/doc/", "share/locale/"];

    //----------------------------
    // Operating
    //----------------------------
    println!("==> Creating archive {}", tar_name);

    // Collect and filter files
    let files: Vec<String> = cbp::find_files(collect_dir, None)?
        .into_iter()
        .filter(|path| !cbp::is_system_file(path))
        .filter(|path| {
            !cleanup || !doc_dirs.iter().any(|prefix| path.starts_with(prefix))
        })
        .collect();

    // Create and compress archive
    let tar_file = std::fs::File::create(&tar_name)?;
    let gz = flate2::write::GzEncoder::new(tar_file, flate2::Compression::default());
    let mut archive = tar::Builder::new(gz);

    // Add files with relative paths
    for path in &files {
        let full_path = collect_dir.join(path);
        archive.append_path_with_name(&full_path, path)?;
    }

    // Finish compression
    archive.finish()?;
    println!("==> Package created: {}", tar_name);

    Ok(())
}
