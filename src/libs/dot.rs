//! Dotfiles management functionality
//!
//! This module provides core functionality for managing dotfiles:
//! - System information detection
//! - Filename parsing with special prefixes
//! - Template rendering with Tera
//! - Path conversion for cross-platform support

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// System information for template rendering
#[derive(Debug, Clone)]
pub struct SystemInfo {
    /// Operating system: linux, macos, windows
    pub os: String,
    /// Architecture: x86_64, aarch64
    pub arch: String,
    /// Hostname
    pub hostname: String,
    /// Username
    pub user: String,
    /// Linux distribution (empty on other platforms)
    pub distro: String,
    /// Environment variables
    pub env: HashMap<String, String>,
}

impl SystemInfo {
    /// Collect system information
    pub fn collect() -> anyhow::Result<Self> {
        let os = crate::get_os_type()?;
        let arch = Self::get_arch();
        let hostname = Self::get_hostname();
        let user = Self::get_user();
        let distro = if os == "linux" {
            Self::get_distro()
        } else {
            String::new()
        };

        // Collect common environment variables
        let mut env = HashMap::new();
        for var in ["HOME", "USER", "USERNAME", "PATH", "SHELL", "EDITOR"] {
            if let Ok(val) = std::env::var(var) {
                env.insert(var.to_string(), val);
            }
        }

        Ok(Self {
            os,
            arch,
            hostname,
            user,
            distro,
            env,
        })
    }

    /// Get architecture
    fn get_arch() -> String {
        match std::env::consts::ARCH {
            "x86_64" => "x86_64".to_string(),
            "aarch64" => "aarch64".to_string(),
            arch => arch.to_string(),
        }
    }

    /// Get hostname
    fn get_hostname() -> String {
        sysinfo::System::host_name().unwrap_or_else(|| "unknown".to_string())
    }

    /// Get current username
    fn get_user() -> String {
        dirs::home_dir()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .or_else(|| std::env::var("USER").ok())
            .or_else(|| std::env::var("USERNAME").ok())
            .unwrap_or_else(|| "unknown".to_string())
    }

    /// Get Linux distribution
    fn get_distro() -> String {
        // Try to read /etc/os-release
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
                    return value.trim_matches('"').to_string();
                }
            }
        }
        "Unknown".to_string()
    }

    /// Convert to Tera context
    pub fn to_context(&self) -> tera::Context {
        let mut context = tera::Context::new();
        context.insert("os", &self.os);
        context.insert("arch", &self.arch);
        context.insert("hostname", &self.hostname);
        context.insert("user", &self.user);
        context.insert("distro", &self.distro);
        context.insert("env", &self.env);
        context
    }
}

/// Parsed dotfile information
#[derive(Debug, Clone)]
pub struct DotfileInfo {
    /// Original filename
    pub original_name: String,
    /// Target filename (after removing prefixes)
    pub target_name: String,
    /// Target directory (home, config, data, cache)
    pub target_dir: TargetDir,
    /// File permissions
    pub permissions: FilePermissions,
    /// Whether this is a template file
    pub is_template: bool,
}

/// Target directory type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetDir {
    /// Home directory (~)
    Home,
    /// Config directory (~/.config or %APPDATA%)
    Config,
    /// Data directory (~/.local/share or %LOCALAPPDATA%)
    Data,
    /// Cache directory (~/.cache or %LOCALAPPDATA%/Temp)
    Cache,
}

/// File permissions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilePermissions {
    /// Default permissions (0644)
    Default,
    /// Private file (0600)
    Private,
    /// Executable file (0755)
    Executable,
}

impl FilePermissions {
    /// Get numeric mode
    pub fn mode(&self) -> u32 {
        match self {
            FilePermissions::Default => 0o644,
            FilePermissions::Private => 0o600,
            FilePermissions::Executable => 0o755,
        }
    }
}

/// Parser for dotfile names
pub struct DotfileParser;

impl DotfileParser {
    /// Parse a dotfile name and extract information
    pub fn parse(name: &str) -> DotfileInfo {
        let mut remaining = name.to_string();
        let mut permissions = FilePermissions::Default;
        let mut target_dir = TargetDir::Home;
        let mut is_template = false;

        // Check for .tmpl suffix
        if let Some(stripped) = remaining.strip_suffix(".tmpl") {
            is_template = true;
            remaining = stripped.to_string();
        }

        // Parse attribute prefixes (private_, executable_)
        // executable_ takes precedence over private_
        let has_private = remaining.starts_with("private_");
        let has_executable = remaining.starts_with("executable_");

        if has_executable {
            permissions = FilePermissions::Executable;
            remaining = remaining.strip_prefix("executable_").unwrap().to_string();
        } else if has_private {
            permissions = FilePermissions::Private;
            remaining = remaining.strip_prefix("private_").unwrap().to_string();
        }

        // Check again after removing first prefix
        if remaining.starts_with("executable_")
            && permissions != FilePermissions::Executable
        {
            permissions = FilePermissions::Executable;
            remaining = remaining.strip_prefix("executable_").unwrap().to_string();
        } else if remaining.starts_with("private_")
            && permissions != FilePermissions::Private
        {
            permissions = FilePermissions::Private;
            remaining = remaining.strip_prefix("private_").unwrap().to_string();
        }

        // Parse directory prefixes
        if remaining.starts_with("dot_config/") {
            target_dir = TargetDir::Config;
            remaining = remaining.strip_prefix("dot_config/").unwrap().to_string();
        } else if remaining.starts_with("xdg_config/") {
            target_dir = TargetDir::Config;
            remaining = remaining.strip_prefix("xdg_config/").unwrap().to_string();
        } else if remaining.starts_with("xdg_data/") {
            target_dir = TargetDir::Data;
            remaining = remaining.strip_prefix("xdg_data/").unwrap().to_string();
        } else if remaining.starts_with("xdg_cache/") {
            target_dir = TargetDir::Cache;
            remaining = remaining.strip_prefix("xdg_cache/").unwrap().to_string();
        } else if remaining.starts_with("dot_") {
            // Single file mode: dot_bashrc -> .bashrc
            remaining = remaining.strip_prefix("dot_").unwrap().to_string();
            remaining = format!(".{}", remaining);
        }

        DotfileInfo {
            original_name: name.to_string(),
            target_name: remaining,
            target_dir,
            permissions,
            is_template,
        }
    }

    /// Infer prefix from a source file path for creating templates
    pub fn infer_prefix(source_path: &Path) -> (String, TargetDir, FilePermissions) {
        let file_name = source_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        let mut prefix = String::new();
        let mut target_dir = TargetDir::Home;
        #[cfg(unix)]
        let mut permissions = FilePermissions::Default;
        #[cfg(not(unix))]
        let permissions = FilePermissions::Default;

        // Check if file is executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = source_path.metadata() {
                let mode = metadata.permissions().mode();
                if mode & 0o111 != 0 {
                    permissions = FilePermissions::Executable;
                    prefix.push_str("executable_");
                }
            }
        }

        // Determine directory prefix
        let parent = source_path.parent();
        if let Some(parent_path) = parent {
            let parent_str = parent_path.to_string_lossy();

            // Check for config directories
            if parent_str.contains(".config") || parent_str.contains("AppData/Roaming") {
                target_dir = TargetDir::Config;
                prefix.push_str("xdg_config/");
            } else if parent_str.contains(".local/share")
                || parent_str.contains("AppData/Local")
            {
                target_dir = TargetDir::Data;
                prefix.push_str("xdg_data/");
            } else if parent_str.contains(".cache")
                || parent_str.contains("AppData/Local/Temp")
            {
                target_dir = TargetDir::Cache;
                prefix.push_str("xdg_cache/");
            } else if file_name.starts_with('.') {
                // Hidden file in home directory
                prefix.push_str("dot_");
            }
        } else if file_name.starts_with('.') {
            prefix.push_str("dot_");
        }

        (prefix, target_dir, permissions)
    }
}

/// Get target directory path based on type and platform
pub fn get_target_dir(target_dir: TargetDir) -> anyhow::Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

    match target_dir {
        TargetDir::Home => Ok(home),
        TargetDir::Config => {
            #[cfg(windows)]
            {
                dirs::data_dir().ok_or_else(|| {
                    anyhow::anyhow!("Could not determine config directory")
                })
            }
            #[cfg(not(windows))]
            {
                Ok(home.join(".config"))
            }
        }
        TargetDir::Data => {
            #[cfg(windows)]
            {
                dirs::data_local_dir()
                    .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))
            }
            #[cfg(not(windows))]
            {
                Ok(home.join(".local/share"))
            }
        }
        TargetDir::Cache => {
            #[cfg(windows)]
            {
                dirs::data_local_dir()
                    .map(|d| d.join("Temp"))
                    .ok_or_else(|| {
                        anyhow::anyhow!("Could not determine cache directory")
                    })
            }
            #[cfg(not(windows))]
            {
                Ok(home.join(".cache"))
            }
        }
    }
}

/// Render template content using Tera
pub fn render_template(
    content: &str,
    context: &tera::Context,
) -> anyhow::Result<String> {
    let mut tera = tera::Tera::default();
    tera.add_raw_template("template", content)?;
    Ok(tera.render("template", context)?)
}

/// Get full target path for a dotfile
pub fn get_target_path(info: &DotfileInfo) -> anyhow::Result<PathBuf> {
    let base_dir = get_target_dir(info.target_dir)?;
    Ok(base_dir.join(&info.target_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dot_prefix() {
        let info = DotfileParser::parse("dot_bashrc");
        assert_eq!(info.target_name, ".bashrc");
        assert_eq!(info.target_dir, TargetDir::Home);
        assert_eq!(info.permissions, FilePermissions::Default);
        assert!(!info.is_template);
    }

    #[test]
    fn test_parse_dot_config() {
        let info = DotfileParser::parse("dot_config/nvim/init.vim");
        assert_eq!(info.target_name, "nvim/init.vim");
        assert_eq!(info.target_dir, TargetDir::Config);
    }

    #[test]
    fn test_parse_xdg_config() {
        let info = DotfileParser::parse("xdg_config/myapp/config");
        assert_eq!(info.target_name, "myapp/config");
        assert_eq!(info.target_dir, TargetDir::Config);
    }

    #[test]
    fn test_parse_xdg_data() {
        let info = DotfileParser::parse("xdg_data/myapp/data.db");
        assert_eq!(info.target_name, "myapp/data.db");
        assert_eq!(info.target_dir, TargetDir::Data);
    }

    #[test]
    fn test_parse_xdg_cache() {
        let info = DotfileParser::parse("xdg_cache/myapp/tmp");
        assert_eq!(info.target_name, "myapp/tmp");
        assert_eq!(info.target_dir, TargetDir::Cache);
    }

    #[test]
    fn test_parse_private() {
        let info = DotfileParser::parse("private_dot_ssh_config");
        assert_eq!(info.target_name, ".ssh_config");
        assert_eq!(info.permissions, FilePermissions::Private);
    }

    #[test]
    fn test_parse_executable() {
        let info = DotfileParser::parse("executable_script.sh");
        assert_eq!(info.target_name, "script.sh");
        assert_eq!(info.permissions, FilePermissions::Executable);
    }

    #[test]
    fn test_parse_private_executable() {
        // executable takes precedence
        let info = DotfileParser::parse("private_executable_dot_myscript");
        assert_eq!(info.target_name, ".myscript");
        assert_eq!(info.permissions, FilePermissions::Executable);
    }

    #[test]
    fn test_parse_template() {
        let info = DotfileParser::parse("dot_bashrc.tmpl");
        assert_eq!(info.target_name, ".bashrc");
        assert!(info.is_template);
    }

    #[test]
    fn test_parse_complex() {
        let info =
            DotfileParser::parse("private_executable_dot_config/myapp/config.tmpl");
        assert_eq!(info.target_name, "myapp/config");
        assert_eq!(info.target_dir, TargetDir::Config);
        assert_eq!(info.permissions, FilePermissions::Executable);
        assert!(info.is_template);
    }

    #[test]
    fn test_render_template() {
        let mut context = tera::Context::new();
        context.insert("os", &"linux");

        let template = r#"{% if os == "linux" %}Linux{% else %}Other{% endif %}"#;
        let result = render_template(template, &context).unwrap();
        assert_eq!(result, "Linux");
    }

    #[test]
    fn test_permissions_mode() {
        assert_eq!(FilePermissions::Default.mode(), 0o644);
        assert_eq!(FilePermissions::Private.mode(), 0o600);
        assert_eq!(FilePermissions::Executable.mode(), 0o755);
    }
}
