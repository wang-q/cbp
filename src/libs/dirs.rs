use std::path::Path;
use std::path::PathBuf;

/// Represents CBP directory structure
pub struct CbpDirs {
    /// Installation directory
    /// Default: ~/.cbp
    pub home: PathBuf,
    /// Binary directory under installation directory
    /// Default: <home>/bin
    pub bin: PathBuf,
    /// Cache directory under installation directory
    /// Default: <home>/cache
    pub cache: PathBuf,
    /// Package records directory under installation directory
    /// Default: <home>/records
    pub records: PathBuf,
}

impl CbpDirs {
    /// Creates a new CbpDirs instance with default home directory (~/.cbp)
    ///
    /// This function will:
    /// 1. Check for custom home directory in config file
    /// 2. Create all required directories if they don't exist
    /// 3. Return error if home directory cannot be determined
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Home directory cannot be determined
    /// - Config file exists but cannot be read or parsed
    /// - Directory creation fails
    pub fn new() -> anyhow::Result<Self> {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
            .join(".cbp");

        Self::from(home)
    }

    pub fn from(home: PathBuf) -> anyhow::Result<Self> {
        let cbp = Self {
            bin: home.join("bin"),
            cache: home.join("cache"),
            records: home.join("records"),
            home,
        };

        // Ensure all directories exist
        std::fs::create_dir_all(&cbp.home)?;
        std::fs::create_dir_all(&cbp.bin)?;
        std::fs::create_dir_all(&cbp.cache)?;
        std::fs::create_dir_all(&cbp.records)?;

        Ok(cbp)
    }

    /// Creates a new CbpDirs instance from the executable's path
    ///
    /// This function will:
    /// 1. Get the executable's directory
    /// 2. Use parent directory as CBP home
    /// 3. Create all required directories if they don't exist
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Cannot get executable path
    /// - Cannot get parent directory
    /// - Directory creation fails
    pub fn from_exe() -> anyhow::Result<Self> {
        let exe_path = dunce::canonicalize(std::env::current_exe()?)?;
        Self::from_exe_path(&exe_path)
    }

    fn from_exe_path(exe_path: &Path) -> anyhow::Result<Self> {
        let home = exe_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Cannot get executable directory"))?
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Cannot get CBP home directory"))?
            .to_path_buf();

        Self::from(home)
    }

    /// Returns the CBP home directory as a string
    pub fn get_home(&self) -> String {
        self.home.to_string_lossy().into_owned()
    }

    /// Install package from a tar.gz file
    ///
    /// # Arguments
    ///
    /// * `pkg_name` - Name of the package
    /// * `pkg_file` - Path to the package tar.gz file
    pub fn install_package(
        &self,
        pkg_name: &str,
        pkg_file: &Path,
    ) -> anyhow::Result<()> {
        println!("==> Installing {}", pkg_name);

        // List files in package
        let record_file = self.records.join(format!("{}.files", pkg_name));
        let file_list = crate::list_archive_files(pkg_file)?;

        // Save file list
        std::fs::write(&record_file, file_list)?;

        // Extract files
        let file = std::fs::File::open(pkg_file)?;
        let gz = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(gz);

        if let Err(e) = archive.unpack(&self.home) {
            std::fs::remove_file(record_file)?;
            return Err(anyhow::anyhow!("    Failed to extract {}: {}", pkg_name, e));
        }

        println!("    Done");
        Ok(())
    }
}

/// Convert relative path to absolute path
pub fn to_absolute_path(path: &str) -> anyhow::Result<std::path::PathBuf> {
    let path_buf = std::path::PathBuf::from(path);
    Ok(if path_buf.is_absolute() {
        path_buf
    } else {
        std::env::current_dir()?.join(path_buf)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(unix)]
    fn test_cbp_dirs_new() -> anyhow::Result<()> {
        let temp_home = tempfile::tempdir()?;
        std::env::set_var("HOME", temp_home.path());

        let dirs = CbpDirs::new()?;
        assert_eq!(dirs.home, temp_home.path().join(".cbp"));
        assert!(dirs.home.exists());
        assert!(dirs.bin.exists());
        assert!(dirs.cache.exists());
        assert!(dirs.records.exists());

        Ok(())
    }

    #[test]
    fn test_cbp_dirs_from() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let dirs = CbpDirs::from(temp_dir.path().to_path_buf())?;

        // Verify directory structure
        assert_eq!(dirs.home, temp_dir.path());
        assert_eq!(dirs.bin, temp_dir.path().join("bin"));
        assert_eq!(dirs.cache, temp_dir.path().join("cache"));
        assert_eq!(dirs.records, temp_dir.path().join("records"));

        // Verify directories exist
        assert!(dirs.home.exists());
        assert!(dirs.bin.exists());
        assert!(dirs.cache.exists());
        assert!(dirs.records.exists());

        Ok(())
    }

    #[test]
    fn test_cbp_dirs_from_exe() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let bin_dir = temp_dir.path().join("bin");
        std::fs::create_dir_all(&bin_dir)?;

        let exe_path = bin_dir.join("cbp.exe");
        std::fs::write(&exe_path, "dummy executable")?;

        let dirs = CbpDirs::from_exe_path(&exe_path)?;

        // Verify directory structure
        assert_eq!(dirs.home, temp_dir.path());
        assert_eq!(dirs.bin, bin_dir);
        assert_eq!(dirs.cache, temp_dir.path().join("cache"));
        assert_eq!(dirs.records, temp_dir.path().join("records"));

        // Verify directories exist
        assert!(dirs.home.exists());
        assert!(dirs.bin.exists());
        assert!(dirs.cache.exists());
        assert!(dirs.records.exists());

        Ok(())
    }

    #[test]
    fn test_to_absolute_path() -> anyhow::Result<()> {
        // Test absolute path
        let abs_path = if cfg!(windows) {
            "C:/absolute/path"
        } else {
            "/absolute/path"
        };
        let result = to_absolute_path(abs_path)?;
        assert_eq!(result, std::path::PathBuf::from(abs_path));

        // Create temporary directory for testing
        let temp_dir = tempfile::tempdir()?;
        let base_dir = temp_dir.path();
        std::env::set_current_dir(&base_dir)?;

        // Test relative paths with different formats
        let test_cases = vec!["relative/path", "./relative/path", "relative/./path"];

        // Verify path construction without checking existence
        for rel_path in test_cases {
            let result = to_absolute_path(rel_path)?;
            let expected = base_dir.join(rel_path);

            // Filter out "private" from components if present
            let result_components: Vec<_> = result
                .components()
                .filter(|c| c.as_os_str() != "private")
                .collect();
            let expected_components: Vec<_> = expected
                .components()
                .filter(|c| c.as_os_str() != "private")
                .collect();

            assert_eq!(result_components, expected_components);
        }

        Ok(())
    }

    #[test]
    fn test_install_real_package() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let cbp_dirs = crate::CbpDirs::from(temp_dir.path().to_path_buf())?;

        // Use test package file
        let test_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/zlib.macos.tar.gz");

        // Test package installation
        cbp_dirs.install_package("zlib", &test_file)?;

        // Verify file list
        let record_file = cbp_dirs.records.join("zlib.files");
        assert!(record_file.exists());
        let file_list = std::fs::read_to_string(record_file)?;
        assert!(file_list.contains("include/zlib.h"));
        assert!(file_list.contains("lib/libz.a"));

        // Verify key files exist
        assert!(cbp_dirs.home.join("include/zlib.h").exists());
        assert!(cbp_dirs.home.join("lib/libz.a").exists());

        Ok(())
    }
}
