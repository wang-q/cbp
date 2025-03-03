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
    CbpDirs::new().map(|dirs| dirs.home.to_string_lossy().into_owned())
}

/// Returns the CBP configuration directory
///
/// This is always ~/.cbp, regardless of the installation directory
///
/// # Errors
///
/// Returns error if home directory cannot be determined
pub fn get_cbp_config_dir() -> anyhow::Result<PathBuf> {
    dirs::home_dir()
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
        let original_home = std::env::var("HOME")?;

        // Set temporary home directory
        std::env::set_var("HOME", temp_home.path());

        // Test default directory structure
        let dirs = CbpDirs::new()?;
        assert!(dirs.home.exists());
        assert!(dirs.bin.exists());
        assert!(dirs.cache.exists());
        assert!(dirs.records.exists());

        // Restore original home directory
        std::env::set_var("HOME", original_home);
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
        let original_home = std::env::var("HOME")?;
        std::env::set_var("HOME", temp_home.path());

        // Create config directory and custom installation directory
        let cbp_dir = temp_home.path().join(".cbp");
        let custom_dir = temp_home.path().join("custom-cbp");
        std::fs::create_dir_all(&cbp_dir)?;

        let config_content = format!(
            r#"
            home = "{}"
        "#,
            custom_dir.display()
        );
        std::fs::write(cbp_dir.join("config.toml"), config_content)?;

        // Test custom directory structure
        let dirs = CbpDirs::new()?;
        assert_eq!(dirs.home, custom_dir);
        assert!(dirs.bin.exists());
        assert!(dirs.cache.exists());
        assert!(dirs.records.exists());

        // Restore environment
        std::env::set_var("HOME", original_home);
        Ok(())
    }

    #[test]
    fn test_get_cbp_config_dir() -> anyhow::Result<()> {
        // Save original HOME environment variable
        let original_home = std::env::var("HOME")?;

        // Test normal case
        {
            let temp_home = tempfile::tempdir()?;
            std::env::set_var("HOME", temp_home.path());

            let config_dir = get_cbp_config_dir()?;
            assert_eq!(config_dir, temp_home.path().join(".cbp"));
        }

        // Restore environment
        std::env::set_var("HOME", original_home);
        Ok(())
    }

    #[test]
    fn test_to_absolute_path() -> anyhow::Result<()> {
        // Test absolute path
        let abs_path = "/absolute/path";
        assert_eq!(to_absolute_path(abs_path)?.to_string_lossy(), abs_path);

        // Test relative path
        let current_dir = std::env::current_dir()?;
        let rel_path = "relative/path";
        assert_eq!(to_absolute_path(rel_path)?, current_dir.join(rel_path));

        Ok(())
    }

    #[test]
    fn test_cbp_dirs_with_invalid_config() -> anyhow::Result<()> {
        let temp_home = tempfile::tempdir()?;
        let original_home = std::env::var("HOME")?;
        std::env::set_var("HOME", temp_home.path());

        // Create invalid config file
        let cbp_dir = temp_home.path().join(".cbp");
        fs::create_dir_all(&cbp_dir)?;
        fs::write(
            cbp_dir.join("config.toml"),
            r#"invalid = "this is valid TOML but has wrong key""#,
        )?;

        // Should fall back to default directory
        let dirs = CbpDirs::new()?;
        assert_eq!(dirs.home, cbp_dir);

        // Restore environment
        std::env::set_var("HOME", original_home);
        Ok(())
    }

    #[test]
    fn test_get_cbp_home() -> anyhow::Result<()> {
        let temp_home = tempfile::tempdir()?;
        let original_home = std::env::var("HOME")?;
        std::env::set_var("HOME", temp_home.path());

        let cbp_home = get_cbp_home()?;
        assert_eq!(cbp_home, temp_home.path().join(".cbp").to_string_lossy());

        // Restore environment
        std::env::set_var("HOME", original_home);
        Ok(())
    }
}
