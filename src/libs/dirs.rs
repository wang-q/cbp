use serde::Deserialize;
use std::path::PathBuf;

/// Represents CBP directory structure
///
/// # Examples
///
/// ```
/// use cbp::CbpDirs;
///
/// let dirs = CbpDirs::new().unwrap();
/// assert!(dirs.home.exists());
/// assert!(dirs.bin.exists());
/// ```
pub struct CbpDirs {
    /// Root directory (~/.cbp)
    pub home: PathBuf,
    /// Binary directory (~/.cbp/bin)
    pub bin: PathBuf,
    /// Cache directory (~/.cbp/cache)
    pub cache: PathBuf,
    /// Package records directory (~/.cbp/records)
    pub records: PathBuf,
}

#[derive(Deserialize, Default)]
struct Config {
    home: Option<String>,
}

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait HomeDirProvider {
    fn home_dir(&self) -> Option<PathBuf>;
}

pub struct DefaultHomeDirProvider;

impl HomeDirProvider for DefaultHomeDirProvider {
    fn home_dir(&self) -> Option<PathBuf> {
        dirs::home_dir()
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

    fn new_with_provider(provider: &dyn HomeDirProvider) -> anyhow::Result<Self> {
        let home = provider
            .home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

        // Read configuration file
        let config_path = home.join(".cbp/config.toml");
        let custom_home = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            config.home.map(PathBuf::from)
        } else {
            None
        };

        // Use configured installation directory or default
        let cbp_home = custom_home.unwrap_or_else(|| home.join(".cbp"));
        Self::from(cbp_home)
    }

    /// Creates a new CbpDirs instance with specified home directory
    ///
    /// # Arguments
    ///
    /// * `home` - The root directory for CBP installation
    ///
    /// # Errors
    ///
    /// Returns error if directory creation fails
    pub fn from(home: PathBuf) -> anyhow::Result<Self> {
        let dirs = Self {
            bin: home.join("bin"),
            cache: home.join("cache"),
            records: home.join("records"),
            home,
        };

        // Ensure all directories exist
        std::fs::create_dir_all(&dirs.home)?;
        std::fs::create_dir_all(&dirs.bin)?;
        std::fs::create_dir_all(&dirs.cache)?;
        std::fs::create_dir_all(&dirs.records)?;

        Ok(dirs)
    }
}

/// Returns the CBP home directory as a string
///
/// # Errors
///
/// Returns error if:
/// - Home directory cannot be determined
/// - Config file exists but cannot be read or parsed
pub fn get_cbp_home() -> anyhow::Result<String> {
    get_cbp_home_with_provider(&DefaultHomeDirProvider)
}

fn get_cbp_home_with_provider(provider: &dyn HomeDirProvider) -> anyhow::Result<String> {
    CbpDirs::new_with_provider(provider)
        .map(|dirs| dirs.home.to_string_lossy().into_owned())
}

/// Returns the CBP configuration directory
pub fn get_cbp_config_dir() -> anyhow::Result<PathBuf> {
    get_cbp_config_dir_with_provider(&DefaultHomeDirProvider)
}

fn get_cbp_config_dir_with_provider(
    provider: &dyn HomeDirProvider,
) -> anyhow::Result<PathBuf> {
    provider
        .home_dir()
        .map(|home| home.join(".cbp"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))
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
        let mut mock = MockHomeDirProvider::new();
        mock.expect_home_dir()
            .return_const(Some(temp_home.path().to_path_buf()));

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
        let mut mock = MockHomeDirProvider::new();
        mock.expect_home_dir()
            .return_const(Some(temp_home.path().to_path_buf()));

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
    fn test_get_cbp_config_dir() -> anyhow::Result<()> {
        let temp_home = tempfile::tempdir()?;
        let mut mock = MockHomeDirProvider::new();
        mock.expect_home_dir()
            .return_const(Some(temp_home.path().to_path_buf()));

        let config_dir = get_cbp_config_dir_with_provider(&mock)?;
        assert_eq!(config_dir, temp_home.path().join(".cbp"));

        Ok(())
    }

    #[test]
    fn test_cbp_dirs_with_invalid_config() -> anyhow::Result<()> {
        let temp_home = tempfile::tempdir()?;
        let mut mock = MockHomeDirProvider::new();
        mock.expect_home_dir()
            .return_const(Some(temp_home.path().to_path_buf()));

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
    fn test_get_cbp_home() -> anyhow::Result<()> {
        let temp_home = tempfile::tempdir()?;
        let mut mock = MockHomeDirProvider::new();
        mock.expect_home_dir()
            .return_const(Some(temp_home.path().to_path_buf()));

        let home = get_cbp_home_with_provider(&mock)?;
        assert_eq!(home, temp_home.path().join(".cbp").to_string_lossy());
        assert!(temp_home.path().join(".cbp").exists());

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
}
