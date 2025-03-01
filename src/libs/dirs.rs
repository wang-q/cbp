use std::path::PathBuf;

/// Represents CBP directory structure
pub struct CbpDirs {
    /// Root directory (~/.cbp)
    pub home: PathBuf,
    /// Binary directory (~/.cbp/bin)
    pub bin: PathBuf,
    /// Cache directory (~/.cbp/cache)
    pub cache: PathBuf,
    /// Package records directory (~/.cbp/binaries)
    pub binaries: PathBuf,
}

impl CbpDirs {
    /// Creates a new CbpDirs instance with default home directory (~/.cbp)
    pub fn new() -> anyhow::Result<Self> {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
            .join(".cbp");

        Self::from(home)
    }

    /// Creates a new CbpDirs instance from a specified home directory
    ///
    /// # Arguments
    ///
    /// * `home` - Base directory for CBP
    pub fn from(home: PathBuf) -> anyhow::Result<Self> {
        let dirs = Self {
            bin: home.join("bin"),
            cache: home.join("cache"),
            binaries: home.join("binaries"),
            home,
        };

        // Ensure all directories exist
        std::fs::create_dir_all(&dirs.home)?;
        std::fs::create_dir_all(&dirs.bin)?;
        std::fs::create_dir_all(&dirs.cache)?;
        std::fs::create_dir_all(&dirs.binaries)?;

        Ok(dirs)
    }
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
        assert!(dirs.binaries.exists());

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
        assert!(dirs.binaries.exists());

        Ok(())
    }
}
