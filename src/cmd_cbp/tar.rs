use clap::*;
use std::path::Path;

pub fn make_subcommand() -> Command {
    Command::new("tar")
        .about("Create package archive")
        .after_help(
            r###"
Create a compressed archive from a directory.

Features:
* Cross-platform archive creation
* Automatic system file filtering
* Platform-specific package naming
* Documentation directory cleanup

Examples:
1. Create package from collect directory:
   cbp tar

2. Create package from custom directory:
   cbp tar path/to/dir

3. Create package with custom name:
   cbp tar --pkg custom_pkg

4. Create package for specific platform:
   cbp tar --os-type macos
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
            Arg::new("pkg")
                .long("pkg")
                .help("Package name")
                .num_args(1)
                .value_name("PKG"),
        )
        .arg(
            Arg::new("os-type")
                .long("os-type")
                .help("Target OS type")
                .num_args(1)
                .value_name("OS"),
        )
}

pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    let collect_dir = Path::new(args.get_one::<String>("dir").unwrap());
    if !collect_dir.exists() {
        return Err(anyhow::anyhow!(
            "Directory not found: {}",
            collect_dir.display()
        ));
    }

    let os_type = if let Some(os) = args.get_one::<String>("os-type") {
        os.to_string()
    } else {
        cbp::get_os_type()?
    };

    // 获取包名
    let pkg = args
        .get_one::<String>("pkg")
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            collect_dir
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "package".to_string())
        });

    let tar_name = format!("{}.{}.tar.gz", pkg, os_type);
    println!("==> Creating package {} for {}", pkg, os_type);

    // 删除不需要的文档目录
    for dir in ["share/info", "share/man", "share/doc", "share/locale"] {
        let _ = std::fs::remove_dir_all(collect_dir.join(dir));
    }

    // 收集文件列表
    let files: Vec<String> = cbp::find_files(collect_dir, None)?
        .into_iter()
        .filter(|path| !cbp::is_system_file(path))
        .collect();

    // Create archive
    let tar_file = std::fs::File::create(&tar_name)?;
    let gz = flate2::write::GzEncoder::new(tar_file, flate2::Compression::default());
    let mut archive = tar::Builder::new(gz);

    // Add files to archive
    for path in &files {
        let full_path = collect_dir.join(path);
        archive.append_path_with_name(&full_path, path)?;
    }

    // Finish compression
    archive.finish()?;

    println!("==> Package created: {}", tar_name);
    Ok(())
}
