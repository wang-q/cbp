use serde::Deserialize;
use std::path::PathBuf;


/// Represents CBP directory structure
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

pub fn get_cbp_home() -> anyhow::Result<String> {
    CbpDirs::new().map(|dirs| dirs.home.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        
        let config_content = format!(r#"
            home = "{}"
        "#, custom_dir.display());
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
}
