use std::io::{BufWriter, Write};
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
/// Returns sorted relative paths
pub fn find_files(dir: &Path, pattern: Option<&str>) -> anyhow::Result<Vec<String>> {
    let pattern = pattern.unwrap_or("*");
    let walker = walkdir::WalkDir::new(dir).into_iter().filter_entry(|e| {
        if !e.file_type().is_file() {
            return true; // 允许继续遍历目录
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
                    .map(|s| s.to_string())
            })
        })
        .collect();

    files.sort();
    Ok(files)
}

/// Check if a file is managed by cbp itself
pub fn is_cbp_file(path: &str) -> bool {
    path == "bin/cbp" || path.starts_with("records/") || path.starts_with("cache/")
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

}
