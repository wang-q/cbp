use serde::Deserialize;
use std::path::Path;
use std::path::PathBuf;

/// Represents CBP directory structure
pub struct CbpDirs {
    /// Installation directory, can be configured via config.toml
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
    /// Configuration directory, always ~/.cbp regardless of installation directory
    pub config: PathBuf,
}

#[derive(Deserialize, Default)]
struct Config {
    home: Option<String>,
}

pub trait HomeDirProvider {
    fn home_dir(&self) -> Option<PathBuf>;
}

struct DefaultHomeDirProvider;

impl HomeDirProvider for DefaultHomeDirProvider {
    fn home_dir(&self) -> Option<PathBuf> {
        dirs::home_dir()
    }
}

#[cfg(test)]
pub struct MockHomeDirProvider {
    home_dir_result: Option<PathBuf>,
}

#[cfg(test)]
impl MockHomeDirProvider {
    pub fn new() -> Self {
        Self {
            home_dir_result: None,
        }
    }

    pub fn expect_home_dir(mut self, result: Option<PathBuf>) -> Self {
        self.home_dir_result = result;
        self
    }
}

#[cfg(test)]
impl HomeDirProvider for MockHomeDirProvider {
    fn home_dir(&self) -> Option<PathBuf> {
        self.home_dir_result.clone()
    }
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
        Self::new_with_provider(&DefaultHomeDirProvider)
    }

    pub fn new_with_provider(provider: &dyn HomeDirProvider) -> anyhow::Result<Self> {
        let home = provider
            .home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

        let config = home.join(".cbp");
        // Read configuration file
        let config_path = config.join("config.toml");
        let custom_home = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            config.home.map(PathBuf::from)
        } else {
            None
        };

        // Use configured installation directory or default
        let cbp_home = custom_home.unwrap_or_else(|| config.clone());
        Self::from_with_config(cbp_home, config)
    }

    fn from_with_config(home: PathBuf, config: PathBuf) -> anyhow::Result<Self> {
        let dirs = Self {
            bin: home.join("bin"),
            cache: home.join("cache"),
            records: home.join("records"),
            config,
            home,
        };

        // Ensure all directories exist
        std::fs::create_dir_all(&dirs.home)?;
        std::fs::create_dir_all(&dirs.bin)?;
        std::fs::create_dir_all(&dirs.cache)?;
        std::fs::create_dir_all(&dirs.records)?;
        std::fs::create_dir_all(&dirs.config)?;

        Ok(dirs)
    }

    pub fn from(home: PathBuf) -> anyhow::Result<Self> {
        Self::from_with_config(home.clone(), home)
    }

    /// Returns the CBP home directory as a string
    pub fn get_home(&self) -> String {
        self.home.to_string_lossy().into_owned()
    }

    /// Returns the CBP configuration directory as a string
    pub fn get_config_dir(&self) -> String {
        self.config.to_string_lossy().into_owned()
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
    use std::fs;

    #[test]
    fn test_cbp_dirs_new() -> anyhow::Result<()> {
        let temp_home = tempfile::tempdir()?;
        let mock = MockHomeDirProvider::new()
            .expect_home_dir(Some(temp_home.path().to_path_buf()));

        let dirs = CbpDirs::new_with_provider(&mock)?;
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

        // Test custom directory structure
        let dirs = CbpDirs::from(temp_dir.path().to_path_buf())?;
        assert!(dirs.home.exists());
        assert!(dirs.bin.exists());
        assert!(dirs.cache.exists());
        assert!(dirs.records.exists());

        Ok(())
    }

    #[test]
    fn test_cbp_dirs_with_config() -> anyhow::Result<()> {
        let temp_home = tempfile::tempdir()?;
        let mock = MockHomeDirProvider::new()
            .expect_home_dir(Some(temp_home.path().to_path_buf()));

        // Create config directory and custom installation directory
        let cbp_dir = temp_home.path().join(".cbp");
        let custom_dir = temp_home.path().join("custom-cbp");
        std::fs::create_dir_all(&cbp_dir)?;

        // Replace backslashes with forward slashes for TOML compatibility
        let config_content = format!(
            r#"
            home = "{}"
        "#,
            custom_dir.display().to_string().replace('\\', "/")
        );
        std::fs::write(cbp_dir.join("config.toml"), config_content)?;

        // Test custom directory structure
        let dirs = CbpDirs::new_with_provider(&mock)?;
        assert_eq!(dirs.home, custom_dir);
        assert!(dirs.bin.exists());
        assert!(dirs.cache.exists());
        assert!(dirs.records.exists());

        Ok(())
    }

    #[test]
    fn test_get_home() -> anyhow::Result<()> {
        let temp_home = tempfile::tempdir()?;
        let mock = MockHomeDirProvider::new()
            .expect_home_dir(Some(temp_home.path().to_path_buf()));

        let dirs = CbpDirs::new_with_provider(&mock)?;
        let home = dirs.get_home();
        assert_eq!(home, temp_home.path().join(".cbp").to_string_lossy());
        assert!(temp_home.path().join(".cbp").exists());

        Ok(())
    }

    #[test]
    fn test_get_config_dir() -> anyhow::Result<()> {
        let temp_home = tempfile::tempdir()?;
        let mock = MockHomeDirProvider::new()
            .expect_home_dir(Some(temp_home.path().to_path_buf()));

        let dirs = CbpDirs::new_with_provider(&mock)?;
        let config_dir = dirs.get_config_dir();
        assert_eq!(config_dir, temp_home.path().join(".cbp").to_string_lossy());
        assert!(temp_home.path().join(".cbp").exists());

        Ok(())
    }

    #[test]
    fn test_cbp_dirs_with_invalid_config() -> anyhow::Result<()> {
        let temp_home = tempfile::tempdir()?;
        let mock = MockHomeDirProvider::new()
            .expect_home_dir(Some(temp_home.path().to_path_buf()));

        // Create invalid config file
        let cbp_dir = temp_home.path().join(".cbp");
        fs::create_dir_all(&cbp_dir)?;
        fs::write(
            cbp_dir.join("config.toml"),
            r#"invalid = "this is valid TOML but has wrong key""#,
        )?;

        // Should fall back to default directory
        let dirs = CbpDirs::new_with_provider(&mock)?;
        assert_eq!(dirs.home, cbp_dir);

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

        // 使用测试包文件
        let test_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/zlib.macos.tar.gz");

        // 测试包安装
        cbp_dirs.install_package("zlib", &test_file)?;

        // 验证文件列表
        let record_file = cbp_dirs.records.join("zlib.files");
        assert!(record_file.exists());
        let file_list = std::fs::read_to_string(record_file)?;
        assert!(file_list.contains("include/zlib.h"));
        assert!(file_list.contains("lib/libz.a"));

        // 验证关键文件存在
        assert!(cbp_dirs.home.join("include/zlib.h").exists());
        assert!(cbp_dirs.home.join("lib/libz.a").exists());

        Ok(())
    }
}
