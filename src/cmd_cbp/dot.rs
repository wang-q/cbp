use clap::*;
use std::path::Path;

use cbp::dot::{DotfileParser, SystemInfo};

pub fn make_subcommand() -> Command {
    Command::new("dot")
        .about("Manage dotfiles with templates")
        .after_help(
            r###"
Manage dotfiles using filename conventions and templates.

Filename prefixes:
  private_      - Set 0600 permissions (sensitive files)
  executable_   - Set 0755 permissions (scripts)
  dot_          - Convert to hidden file (~/.name)
  dot_config/   - Config directory (~/.config/)
  xdg_config/   - Cross-platform config
  xdg_data/     - Cross-platform data
  xdg_cache/    - Cross-platform cache
  .tmpl         - Template file (Tera/Jinja2 syntax)

Examples:
1. Create template from existing config:
   cbp dot ~/.bashrc --dir ~/dotfiles/
   # Creates: ~/dotfiles/dot_bashrc.tmpl

2. Preview template:
   cbp dot ~/dotfiles/dot_bashrc.tmpl

3. Apply template:
   cbp dot -a ~/dotfiles/dot_bashrc.tmpl

4. Apply all templates:
   for f in ~/dotfiles/dot_*; do cbp dot -a "$f"; done

5. Export config to archive:
   cbp dot ~/.config/nvim --tar nvim.tar.gz

6. Apply archive:
   cbp dot -a nvim.tar.gz

"###,
        )
        .arg(
            Arg::new("source")
                .help("Source file(s), template file(s), or archive(s)")
                .required(true)
                .num_args(1..)
                .value_name("SOURCE"),
        )
        .arg(
            Arg::new("apply")
                .long("apply")
                .short('a')
                .help("Actually apply changes (default: preview only)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Show verbose output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .help("Create template from existing config")
                .num_args(1)
                .value_name("DIR"),
        )
        .arg(
            Arg::new("tar")
                .long("tar")
                .help("Export to tar.gz archive")
                .num_args(1)
                .value_name("FILE"),
        )
}

pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    let sources: Vec<String> = args
        .get_many::<String>("source")
        .unwrap()
        .cloned()
        .collect();
    let apply = args.get_flag("apply");
    let verbose = args.get_flag("verbose");

    // Check for conflicting options
    let has_dir = args.contains_id("dir");
    let has_tar = args.contains_id("tar");

    if has_dir && apply {
        return Err(anyhow::anyhow!(
            "Cannot use --apply and --dir together. --dir is for creating templates, --apply is for applying them."
        ));
    }

    if has_dir && has_tar {
        return Err(anyhow::anyhow!("Cannot use --dir and --tar together."));
    }

    // Dispatch to appropriate handler
    if has_dir {
        // Create mode: from existing config
        if sources.len() > 1 {
            return Err(anyhow::anyhow!(
                "Cannot use multiple sources with --dir. Please specify a single source."
            ));
        }
        let template_dir = args.get_one::<String>("dir").unwrap();
        create_template(std::path::Path::new(&sources[0]), template_dir, verbose)
    } else if has_tar {
        // Export mode: to tar.gz
        let tar_file = args.get_one::<String>("tar").unwrap();
        let source_paths: Vec<&Path> =
            sources.iter().map(|s| std::path::Path::new(s)).collect();
        export_archive(&source_paths, tar_file, verbose)
    } else {
        // Apply mode: process each source
        for source in &sources {
            let source_path = std::path::Path::new(source);
            if source.ends_with(".tar.gz") || source.ends_with(".tgz") {
                apply_archive(source_path, apply, verbose)?;
            } else {
                apply_template(source_path, apply, verbose)?;
            }
        }
        Ok(())
    }
}

/// Create a template from an existing config file
fn create_template(
    source_path: &Path,
    template_dir: &str,
    verbose: bool,
) -> anyhow::Result<()> {
    if !source_path.exists() {
        return Err(anyhow::anyhow!(
            "Source file not found: {}",
            source_path.display()
        ));
    }

    let template_dir = std::path::Path::new(template_dir);
    std::fs::create_dir_all(template_dir)?;

    // Infer prefix from source path
    let (prefix, target_dir, permissions) = DotfileParser::infer_prefix(source_path);

    // Read source content
    let content = std::fs::read_to_string(source_path)?;

    // Determine template name
    let file_name = source_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    // Remove leading dot if present (we'll add dot_ prefix)
    let base_name = if file_name.starts_with('.') {
        &file_name[1..]
    } else {
        &file_name
    };

    // Build template filename
    let mut template_name: String = prefix;
    template_name.push_str(base_name);
    template_name.push_str(".tmpl");

    let template_path = template_dir.join(&template_name);

    // Create parent directories if needed
    if let Some(parent) = template_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Write template
    std::fs::write(&template_path, content)?;

    if verbose {
        println!("Source: {}", source_path.display());
        println!("Template: {}", template_path.display());
        println!("Target dir: {:?}", target_dir);
        println!("Permissions: {:?}", permissions);
    } else {
        println!("Created: {}", template_path.display());
    }

    Ok(())
}

/// Apply a template file
fn apply_template(
    template_path: &Path,
    apply: bool,
    verbose: bool,
) -> anyhow::Result<()> {
    if !template_path.exists() {
        return Err(anyhow::anyhow!(
            "Template file not found: {}",
            template_path.display()
        ));
    }

    // Get the filename from the path
    let file_name = template_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .ok_or_else(|| anyhow::anyhow!("Invalid template path"))?;

    // Parse the filename
    let info = DotfileParser::parse(&file_name);

    // Read template content
    let content = std::fs::read_to_string(template_path)?;

    // Render if it's a template
    let final_content = if info.is_template {
        let sys_info = SystemInfo::collect()?;
        let context = sys_info.to_context();
        cbp::dot::render_template(&content, &context)?
    } else {
        content
    };

    // Get target path
    let target_path = cbp::dot::get_target_path(&info)?;

    if verbose {
        println!("Template: {}", template_path.display());
        println!("Target: {}", target_path.display());
        println!(
            "Permissions: {:?} (0{:o})",
            info.permissions,
            info.permissions.mode()
        );
        println!("Is template: {}", info.is_template);
    }

    if apply {
        // Create parent directories
        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Remove existing file or symlink before writing
        // Note: std::fs::write follows symlinks and writes to the target file,
        // which is not the desired behavior. We need to remove the symlink first.
        if target_path.exists() || target_path.is_symlink() {
            std::fs::remove_file(&target_path)?;
        }

        // Write file
        std::fs::write(&target_path, final_content)?;

        // Set permissions on Unix only if explicitly specified
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if info.permissions.is_explicit() {
                let permissions =
                    std::fs::Permissions::from_mode(info.permissions.mode());
                std::fs::set_permissions(&target_path, permissions)?;
            }
        }

        println!(
            "Applied: {} -> {}",
            template_path.display(),
            target_path.display()
        );
    } else {
        // Preview mode
        println!(
            "==> Preview: {} -> {}",
            template_path.display(),
            target_path.display()
        );
        println!(
            "    Permissions: {:?} (0{:o})",
            info.permissions,
            info.permissions.mode()
        );

        if info.is_template {
            println!("\n--- Rendered content ---");
        } else {
            println!("\n--- Content ---");
        }
        println!("{}", final_content);
        println!("------------------------");
        println!("\nUse -a or --apply to actually apply this template.");
    }

    Ok(())
}

/// Apply a tar.gz archive
fn apply_archive(archive_path: &Path, apply: bool, verbose: bool) -> anyhow::Result<()> {
    if !archive_path.exists() {
        return Err(anyhow::anyhow!(
            "Archive not found: {}",
            archive_path.display()
        ));
    }

    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

    // List files in archive
    let file_list = cbp::list_archive_files(archive_path)?;
    let files: Vec<&str> = file_list.lines().filter(|l: &&str| !l.is_empty()).collect();

    if verbose {
        println!("Archive: {}", archive_path.display());
        println!("Target directory: {}", home.display());
        println!("Files in archive:");
        for file in &files {
            println!("  {}", file);
        }
    }

    if apply {
        // Extract archive
        let file = std::fs::File::open(archive_path)?;
        let gz = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(gz);

        archive.unpack(&home)?;

        println!(
            "Applied archive: {} -> {}",
            archive_path.display(),
            home.display()
        );
    } else {
        // Preview mode
        println!(
            "==> Preview: {} -> {}",
            archive_path.display(),
            home.display()
        );
        println!("Files to extract:");
        for file in &files {
            let target = home.join(file);
            let status = if target.exists() {
                " [will overwrite]"
            } else {
                ""
            };
            println!("  {}{}", file, status);
        }
        println!("\nUse -a or --apply to actually extract this archive.");
    }

    Ok(())
}

/// Export config to tar.gz archive
fn export_archive(
    source_paths: &[&Path],
    tar_file: &str,
    verbose: bool,
) -> anyhow::Result<()> {
    for source_path in source_paths {
        if !source_path.exists() {
            return Err(anyhow::anyhow!(
                "Source not found: {}",
                source_path.display()
            ));
        }
    }

    let tar_path = std::path::Path::new(tar_file);
    let cbp = std::env::current_exe()?.display().to_string();

    // Create temporary directory to collect all sources
    let temp_dir = tempfile::tempdir()?;

    // Copy all sources to temporary directory
    for source_path in source_paths {
        let dest_name = source_path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid source path"))?;
        let dest_path = temp_dir.path().join(dest_name);

        if source_path.is_file() {
            std::fs::copy(source_path, &dest_path)?;
        } else {
            cbp::copy_dir_all(source_path, &dest_path)?;
        }
    }

    // Use cbp tar command to create archive
    let output = std::process::Command::new(&cbp)
        .args(["tar", temp_dir.path().to_str().unwrap(), "-o"])
        .arg(tar_path)
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Failed to create archive: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    if verbose {
        for source_path in source_paths {
            println!("Source: {}", source_path.display());
        }
        println!("Archive: {}", tar_path.display());
    } else {
        println!("Created: {}", tar_path.display());
    }

    Ok(())
}
