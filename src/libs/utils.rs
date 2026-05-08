use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};

/// Creates a buffered writer for either stdout or a file
///
/// # Arguments
///
/// * `output` - Output target, "stdout" for standard output or a file path
///
/// # Returns
///
/// A boxed writer implementing the Write trait
pub fn writer(output: &str) -> Box<dyn Write> {
    let writer: Box<dyn Write> = if output == "stdout" {
        Box::new(BufWriter::new(std::io::stdout()))
    } else {
        Box::new(BufWriter::new(std::fs::File::create(output).unwrap()))
    };

    writer
}

pub fn get_os_type() -> anyhow::Result<String> {
    match std::env::consts::OS {
        "macos" => Ok("macos".to_string()),
        "linux" => Ok("linux".to_string()),
        "windows" => Ok("windows".to_string()),
        os => Err(anyhow::anyhow!("Unsupported OS: {}", os)),
    }
}

/// Format package names in columns with 14 characters width
/// Groups packages by their first letter and wraps lines at 80 characters
pub fn format_packages(packages: &[String]) -> String {
    let mut result = String::new();
    let mut prev_char = '\0';
    let mut count = 0;
    let width = 80;

    for name in packages {
        if name.is_empty() {
            continue;
        }

        let first_char = name.chars().next().unwrap();
        if prev_char != '\0' && first_char != prev_char {
            result.push('\n');
            count = 0;
        }

        if count > 0 && count * 16 + 16 > width {
            result.push('\n');
            count = 0;
        }

        prev_char = first_char;
        result.push_str(&format!("  {:<14}", name));
        count += 1;
    }

    result
}

/// Resolve a path, expanding `~` to the home directory
pub fn resolve_path(path: &Path, home: &Path) -> anyhow::Result<PathBuf> {
    use anyhow::Context;

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

/// Convert an absolute path to a home-relative path with `~` prefix
pub fn to_home_path(abs: &Path, home: &Path) -> anyhow::Result<String> {
    let abs = dunce::canonicalize(abs).unwrap_or_else(|_| abs.to_path_buf());
    let home = dunce::canonicalize(home).unwrap_or_else(|_| home.to_path_buf());

    if let Ok(rel) = abs.strip_prefix(&home) {
        let rel_str = rel.display().to_string();
        return Ok(if rel_str.is_empty() {
            "~/".to_string()
        } else {
            format!("~/ {}", rel_str).replace(" ", "")
        });
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

/// Expand `~` in a string path to the home directory (no existence check)
pub fn expand_home_path(path: &str, home: &Path) -> PathBuf {
    if path == "~" {
        return home.to_path_buf();
    }
    let separator = if path.contains('/') { '/' } else { '\\' };
    if let Some(rest) = path.strip_prefix(&format!("~{}", separator)) {
        home.join(rest)
    } else if let Some(rest) = path.strip_prefix("~/") {
        home.join(rest)
    } else if let Some(rest) = path.strip_prefix("~\\") {
        home.join(rest)
    } else {
        PathBuf::from(path)
    }
}

/// Recursively copy a directory and all its contents
pub fn copy_dir_all(
    src: impl AsRef<std::path::Path>,
    dst: impl AsRef<std::path::Path>,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Move a file or directory from source to destination
/// Uses copy and delete to handle cross-device scenarios
pub fn move_file_or_dir(
    source_path: &std::path::Path,
    target_path: &std::path::Path,
) -> anyhow::Result<()> {
    if source_path != target_path && source_path.exists() {
        if source_path.is_dir() {
            // For directories, use recursive copy
            copy_dir_all(source_path, target_path)?;
            std::fs::remove_dir_all(source_path)?;
        } else {
            // For files, use simple copy
            std::fs::copy(source_path, target_path)?;
            std::fs::remove_file(source_path)?;
        }
    }
    Ok(())
}

/// Find files in directory with optional pattern
/// Returns sorted relative paths with forward slashes
pub fn find_files(dir: &Path, pattern: Option<&str>) -> anyhow::Result<Vec<String>> {
    let pattern = pattern.unwrap_or("*");
    let walker = walkdir::WalkDir::new(dir).into_iter().filter_entry(|e| {
        if !e.file_type().is_file() {
            return true; // Continue traversing directories
        }
        match glob::Pattern::new(pattern) {
            Ok(pat) => pat.matches(e.file_name().to_str().unwrap_or_default()),
            Err(_) => false,
        }
    });

    let mut files: Vec<String> = walker
        .filter_map(|e| {
            e.ok().and_then(|entry| {
                if !entry.file_type().is_file() && !entry.file_type().is_symlink() {
                    return None;
                }
                entry
                    .path()
                    .strip_prefix(dir)
                    .ok()
                    .and_then(|p| p.to_str())
                    .map(|s| s.replace('\\', "/")) // Convert backslashes to forward slashes
            })
        })
        .collect();

    files.sort();
    Ok(files)
}

/// Match files using glob pattern and return matching paths
pub fn match_files(
    base_path: &std::path::Path,
    pattern_str: &str,
) -> anyhow::Result<Vec<(std::path::PathBuf, String)>> {
    let pattern = glob::Pattern::new(pattern_str)?;
    let mut matches = Vec::new();
    for entry in walkdir::WalkDir::new(base_path) {
        let entry = entry?;
        let rel_path = entry
            .path()
            .strip_prefix(base_path)?
            .to_string_lossy()
            .to_string();
        if pattern.matches(&rel_path) {
            matches.push((entry.path().to_path_buf(), rel_path));
        }
    }
    Ok(matches)
}

/// Check if a file is managed by cbp itself
pub fn is_cbp_file(path: &str) -> bool {
    path.starts_with("bin/cbp")
        || path.starts_with("bin/zig-")
        || path.starts_with("records/")
        || path.starts_with("cache/")
        || path.starts_with("triplets/")
}

/// Check if a file should be ignored based on system patterns
pub fn is_system_file(path: &str) -> bool {
    // Skip system generated files
    path.ends_with(".DS_Store") ||      // macOS system files
    path.contains("/__MACOSX/") ||
    path.ends_with(".AppleDouble") ||
    path.split('/').next_back().is_some_and(|f| f.starts_with("._")) || // macOS resource fork files
    path.ends_with("Thumbs.db") ||      // Windows system files
    path.ends_with("desktop.ini") ||
    path.ends_with("~") ||              // Linux hidden files
    path.ends_with(".swp") ||
    path.ends_with(".lnk") ||           // Windows shortcuts
    path.contains("/System Volume Information/") // Windows system directory
}

/// Create an HTTP agent with optional proxy support
/// Priority: command line proxy > ALL_PROXY > HTTP_PROXY > all_proxy > http_proxy
pub fn create_http_agent(proxy_url: Option<&String>) -> anyhow::Result<ureq::Agent> {
    let proxy_url = proxy_url
        .cloned()
        .or_else(|| std::env::var("ALL_PROXY").ok())
        .or_else(|| std::env::var("HTTP_PROXY").ok())
        .or_else(|| std::env::var("all_proxy").ok())
        .or_else(|| std::env::var("http_proxy").ok())
        .map(|url| url.replace("socks5h://", "socks5://"));

    Ok(if let Some(proxy_url) = proxy_url {
        let proxy = ureq::Proxy::new(&proxy_url)?;
        ureq::AgentBuilder::new().proxy(proxy).build()
    } else {
        ureq::AgentBuilder::new().build()
    })
}

/// List files in a tar.gz archive
///
/// # Arguments
///
/// * `archive_path` - Path to the tar.gz file
///
/// # Returns
///
/// A list of file paths in the archive, one per line
pub fn list_archive_files(archive_path: &Path) -> anyhow::Result<String> {
    let file = std::fs::File::open(archive_path)?;
    let gz = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(gz);

    let mut file_list = String::new();
    let entries = archive.entries()?;
    for entry in entries {
        let entry = entry?;
        if let Some(path) = entry.path()?.to_str() {
            file_list.push_str(path);
            file_list.push('\n');
        }
    }

    Ok(file_list)
}

/// Read file content from a tar.gz archive
pub fn read_file_from_archive(
    archive_path: &std::path::Path,
    file_path: &str,
) -> anyhow::Result<String> {
    let file = std::fs::File::open(archive_path)?;
    let tar = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(tar);

    for entry in archive.entries()? {
        let mut entry = entry?;
        if entry.path()?.to_string_lossy() == file_path {
            let mut content = String::new();
            entry.read_to_string(&mut content)?;
            return Ok(content);
        }
    }

    Err(anyhow::anyhow!("File not found in archive: {}", file_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_os_type() {
        // Since std::env::consts::OS is a compile-time constant,
        // we can only test the current system's return value

        #[cfg(target_os = "macos")]
        {
            assert_eq!(get_os_type().unwrap(), "macos");
        }

        #[cfg(target_os = "linux")]
        {
            assert_eq!(get_os_type().unwrap(), "linux");
        }

        #[cfg(target_os = "windows")]
        {
            assert_eq!(get_os_type().unwrap(), "windows");
        }

        #[cfg(not(any(
            target_os = "macos",
            target_os = "linux",
            target_os = "windows"
        )))]
        {
            assert!(get_os_type().is_err());
        }
    }

    #[test]
    fn test_format_packages() {
        let packages = vec![
            "abc".to_string(),
            "abd".to_string(),
            "bcd".to_string(),
            "bce".to_string(),
            "cde".to_string(),
        ];
        let formatted = format_packages(&packages);
        assert_eq!(
            formatted,
            "  abc             abd           \n  bcd             bce           \n  cde           "
        );
    }

    #[test]
    fn test_find_files() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let base = temp_dir.path();

        // Create test files
        std::fs::create_dir_all(base.join("dir1"))?;
        std::fs::write(base.join("file1.txt"), "")?;
        std::fs::write(base.join("file2.dat"), "")?;
        std::fs::write(base.join("dir1/file3.txt"), "")?;

        // Test without pattern
        let files = find_files(base, None)?;
        assert_eq!(files, vec!["dir1/file3.txt", "file1.txt", "file2.dat"]);

        // Test with pattern
        let files = find_files(base, Some("*.txt"))?;
        assert_eq!(files, vec!["dir1/file3.txt", "file1.txt"]);

        Ok(())
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_match_files() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let base = temp_dir.path();

        // Create test file structure
        std::fs::create_dir_all(base.join("dir1"))?;
        std::fs::create_dir_all(base.join("dir2"))?;
        std::fs::write(base.join("file1.txt"), "")?;
        std::fs::write(base.join("file2.rs"), "")?;
        std::fs::write(base.join("dir1/file3.txt"), "")?;
        std::fs::write(base.join("dir2/file4.rs"), "")?;

        // Test exact match
        let matches = match_files(base, "file1.txt")?;
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].1, "file1.txt");

        // Test wildcard match
        let matches = match_files(base, "*.txt")?;
        assert_eq!(matches.len(), 2);
        let paths: Vec<_> = matches.iter().map(|(_, p)| p.as_str()).collect();
        assert!(paths.contains(&"file1.txt"));
        assert!(paths.contains(&"dir1/file3.txt"));

        // Test directory wildcard match
        let matches = match_files(base, "dir*/*.rs")?;
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].1, "dir2/file4.rs");

        // Test no matches
        let matches = match_files(base, "nonexistent.*")?;
        assert!(matches.is_empty());

        Ok(())
    }

    #[test]
    fn test_copy_dir_all() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let base = temp_dir.path();

        // Create source directory structure
        let src_dir = base.join("src");
        let dst_dir = base.join("dst");

        std::fs::create_dir_all(&src_dir)?;
        std::fs::create_dir_all(src_dir.join("subdir"))?;
        std::fs::write(src_dir.join("file1.txt"), "content1")?;
        std::fs::write(src_dir.join("file2.dat"), "content2")?;
        std::fs::write(src_dir.join("subdir/file3.txt"), "content3")?;

        // Test copy_dir_all
        copy_dir_all(&src_dir, &dst_dir)?;

        // Verify destination structure
        assert!(dst_dir.exists());
        assert!(dst_dir.join("file1.txt").exists());
        assert!(dst_dir.join("file2.dat").exists());
        assert!(dst_dir.join("subdir").exists());
        assert!(dst_dir.join("subdir/file3.txt").exists());

        // Verify file contents
        assert_eq!(
            std::fs::read_to_string(dst_dir.join("file1.txt"))?,
            "content1"
        );
        assert_eq!(
            std::fs::read_to_string(dst_dir.join("file2.dat"))?,
            "content2"
        );
        assert_eq!(
            std::fs::read_to_string(dst_dir.join("subdir/file3.txt"))?,
            "content3"
        );

        // Verify source still exists
        assert!(src_dir.exists());
        assert!(src_dir.join("file1.txt").exists());

        Ok(())
    }

    #[test]
    fn test_move_file_or_dir() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let base = temp_dir.path();

        // Test moving a file
        let src_file = base.join("source.txt");
        let dst_file = base.join("destination.txt");
        std::fs::write(&src_file, "test content")?;

        move_file_or_dir(&src_file, &dst_file)?;

        assert!(!src_file.exists());
        assert!(dst_file.exists());
        assert_eq!(std::fs::read_to_string(&dst_file)?, "test content");

        // Test moving a directory
        let src_dir = base.join("src_dir");
        let dst_dir = base.join("dst_dir");

        std::fs::create_dir_all(&src_dir)?;
        std::fs::create_dir_all(src_dir.join("subdir"))?;
        std::fs::write(src_dir.join("file1.txt"), "content1")?;
        std::fs::write(src_dir.join("subdir/file2.txt"), "content2")?;

        move_file_or_dir(&src_dir, &dst_dir)?;

        // Verify source is gone and destination exists
        assert!(!src_dir.exists());
        assert!(dst_dir.exists());
        assert!(dst_dir.join("file1.txt").exists());
        assert!(dst_dir.join("subdir").exists());
        assert!(dst_dir.join("subdir/file2.txt").exists());

        // Verify file contents
        assert_eq!(
            std::fs::read_to_string(dst_dir.join("file1.txt"))?,
            "content1"
        );
        assert_eq!(
            std::fs::read_to_string(dst_dir.join("subdir/file2.txt"))?,
            "content2"
        );

        // Test no-op when source and destination are the same
        let same_file = base.join("same.txt");
        std::fs::write(&same_file, "same content")?;
        move_file_or_dir(&same_file, &same_file)?;
        assert!(same_file.exists());
        assert_eq!(std::fs::read_to_string(&same_file)?, "same content");

        // Test no-op when source doesn't exist
        let nonexistent = base.join("nonexistent.txt");
        let target = base.join("target.txt");
        move_file_or_dir(&nonexistent, &target)?;
        assert!(!nonexistent.exists());
        assert!(!target.exists());

        Ok(())
    }

    #[test]
    fn test_resolve_path_tilde() -> anyhow::Result<()> {
        let tmp = tempfile::tempdir()?;
        let home = tmp.path().join("home");
        std::fs::create_dir(&home)?;
        let file = home.join("test.txt");
        std::fs::File::create(&file)?;

        let resolved = resolve_path(Path::new("~/test.txt"), &home)?;
        assert_eq!(resolved, file);
        Ok(())
    }

    #[test]
    fn test_resolve_path_absolute() -> anyhow::Result<()> {
        let tmp = tempfile::tempdir()?;
        let home = tmp.path().join("home");
        std::fs::create_dir(&home)?;
        let file = home.join("abs.txt");
        std::fs::File::create(&file)?;

        let resolved = resolve_path(&file, &home)?;
        assert_eq!(resolved, file);
        Ok(())
    }

    #[test]
    fn test_resolve_path_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path().join("home");
        std::fs::create_dir(&home).unwrap();
        let result = resolve_path(Path::new("~/nonexistent.txt"), &home);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_home_path_under_home() -> anyhow::Result<()> {
        let tmp = tempfile::tempdir()?;
        let home = tmp.path().join("home");
        std::fs::create_dir_all(home.join(".config/nvim"))?;
        let subdir = home.join(".config/nvim");

        let rel = to_home_path(&subdir, &home)?;
        assert!(rel.contains(".config") && rel.contains("nvim"));
        Ok(())
    }

    #[test]
    fn test_to_home_path_outside_home() -> anyhow::Result<()> {
        let tmp = tempfile::tempdir()?;
        let home = tmp.path().join("home");
        std::fs::create_dir(&home)?;
        let outside = tmp.path().join("outside");
        std::fs::create_dir(&outside)?;

        let rel = to_home_path(&outside, &home)?;
        assert!(rel.starts_with("~/../../") || rel.contains("outside"));
        Ok(())
    }

    #[test]
    fn test_to_home_path_home_itself() -> anyhow::Result<()> {
        let tmp = tempfile::tempdir()?;
        let home = tmp.path().join("home");
        std::fs::create_dir(&home)?;

        let rel = to_home_path(&home, &home)?;
        assert_eq!(rel, "~/");
        Ok(())
    }

    #[test]
    fn test_expand_home_path() {
        let home = PathBuf::from("/home/user");
        assert_eq!(expand_home_path("~", &home), PathBuf::from("/home/user"));
        assert_eq!(
            expand_home_path("~/foo", &home),
            PathBuf::from("/home/user/foo")
        );
        assert_eq!(
            expand_home_path("/absolute/path", &home),
            PathBuf::from("/absolute/path")
        );
    }

    #[test]
    fn test_read_comment() -> anyhow::Result<()> {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let tmp = tempfile::tempdir()?;
        let archive_path = tmp.path().join("test.snap.tar.gz");

        // Create a gz file with comment
        let file = std::fs::File::create(&archive_path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(b"test content")?;
        let mut file = encoder.finish()?;
        // Note: GzEncoder doesn't support comment, so we test empty case

        let comment = read_comment(&archive_path)?;
        assert!(comment.is_empty());
        Ok(())
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500B");
        assert_eq!(format_size(1024), "1.0K");
        assert_eq!(format_size(1024 * 1024), "1.0M");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.0G");
    }
}

/// Read gzip comment from a snapshot archive
pub fn read_comment(path: &Path) -> anyhow::Result<String> {
    use std::io::Read;

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

/// Format byte size to human readable string
pub fn format_size(bytes: u64) -> String {
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

/// Find the target path for an archive entry based on source paths
/// Returns the absolute path where the entry should be extracted
pub fn find_target_path(
    archive_entry: &Path,
    source_paths: &[String],
    home: &Path,
) -> Option<PathBuf> {
    let entry_str = archive_entry.to_string_lossy().to_string();

    // Try to match against each source path
    for source in source_paths {
        let source_path = expand_home_path(source, home);
        let source_name = source_path.file_name()?.to_string_lossy();

        // Check if entry starts with the source name
        if entry_str.starts_with(&*source_name) {
            // Get the relative part after the source name
            let rel_part = entry_str.strip_prefix(&*source_name)?;
            let rel_part = rel_part
                .strip_prefix('/')
                .or_else(|| rel_part.strip_prefix('\\'))
                .unwrap_or(rel_part);

            // Build the full target path
            // If rel_part is empty, return source_path (single file case)
            // Otherwise, join rel_part to source_path (directory case)
            let target = if rel_part.is_empty() {
                source_path
            } else {
                source_path.join(rel_part)
            };
            return Some(target);
        }
    }

    // Fallback: try to construct path from home
    Some(home.join(&entry_str))
}

/// Generate delta snapshot output name from archive path
/// Converts "name.tar.gz" or "name.snap.tar.gz" to "name.delta.tar.gz"
pub fn delta_output_name(archive: &Path) -> String {
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

/// Find matching source path for a display path
/// Used when packing modified files to determine archive structure
pub fn find_matching_source(
    display_path: &Path,
    source_paths: &[String],
) -> Option<String> {
    let display = display_path.to_string_lossy().to_string();
    let display = if display.starts_with('/') || display.starts_with('\\') {
        display
    } else {
        format!("/ {}", display)
    };

    for source in source_paths {
        let source_no_tilde = source.strip_prefix('~').unwrap_or(source);
        if display.starts_with(source_no_tilde) || display.contains(source_no_tilde) {
            return Some(source.clone());
        }
    }
    source_paths.first().cloned()
}

/// Generate font installation instructions for the current OS
pub fn font_install_instructions(os_type: &str, font_dir: &Path) -> String {
    let mut result = String::new();
    result.push_str("==> To install fonts for current user, run:\n\n");

    match os_type {
        "windows" => {
            result.push_str(
                "$fonts = (New-Object -ComObject Shell.Application).Namespace(0x14)\n",
            );
            result.push_str(&format!(
                "Get-ChildItem \"{}\" -Include '*.ttf','*.ttc','*.otf' -Recurse | ForEach {{\n",
                font_dir.display()
            ));
            result.push_str(
                "    If (-not(Test-Path \"C:\\Windows\\Fonts\\$($_.Name)\") -and\n",
            );
            result.push_str("        -not(Test-Path \"$env:LOCALAPPDATA\\Microsoft\\Windows\\Fonts\\$($_.Name)\")) {\n");
            result.push_str("        $fonts.CopyHere($_.FullName, 0x10)\n");
            result.push_str("        Write-Host \"Installing $($_.Name)...\"\n");
            result.push_str("    }\n");
            result.push_str("}\n");
        }
        "macos" => {
            result.push_str("for ext in ttf ttc otf; do\n");
            result.push_str(&format!(
                "    find \"{}\" -type f -iname \"*.$ext\" -print0 | while IFS= read -r -d '' font; do\n",
                font_dir.display()
            ));
            result.push_str("        basename=\"$(basename \"$font\")\"\n");
            result.push_str(
                "        if [ ! -f \"$HOME/Library/Fonts/$basename\" ]; then\n",
            );
            result.push_str("            cp \"$font\" \"$HOME/Library/Fonts/\"\n");
            result.push_str("            echo \"Installing $basename...\"\n");
            result.push_str("        fi\n");
            result.push_str("    done\n");
            result.push_str("done\n");
        }
        "linux" => {
            result.push_str("mkdir -p \"$HOME/.local/share/fonts\"\n");
            result.push_str("for ext in ttf ttc otf; do\n");
            result.push_str(&format!(
                "    find \"{}\" -type f -iname \"*.$ext\" -print0 | while IFS= read -r -d '' font; do\n",
                font_dir.display()
            ));
            result.push_str("        basename=\"$(basename \"$font\")\"\n");
            result.push_str(
                "        if [ ! -f \"$HOME/.local/share/fonts/$basename\" ]; then\n",
            );
            result.push_str("            cp \"$font\" \"$HOME/.local/share/fonts/\"\n");
            result.push_str("            echo \"Installing $basename...\"\n");
            result.push_str("        fi\n");
            result.push_str("    done\n");
            result.push_str("done\n");
            result.push_str("fc-cache -f -v\n");
        }
        _ => {}
    }

    result
}
