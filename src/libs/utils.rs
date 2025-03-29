use std::io::{BufWriter, Read, Write};
use std::path::Path;

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
    path.split('/').last().is_some_and(|f| f.starts_with("._")) || // macOS resource fork files
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
            result.push_str(&format!("for ext in ttf ttc otf; do\n",));
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
            result.push_str(&format!("for ext in ttf ttc otf; do\n",));
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
