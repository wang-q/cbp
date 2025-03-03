use assert_cmd::prelude::*;
use std::process::Command;

#[test]
#[cfg(unix)]
fn command_init_default() -> anyhow::Result<()> {
    use std::fs;
    use tempfile::TempDir;

    // Create temporary home directory
    let temp_home = TempDir::new()?;
    let original_home = std::env::var("HOME")?;
    std::env::set_var("HOME", temp_home.path());

    // Create shell config files with initial content
    for config in [".bashrc", ".bash_profile", ".zshrc"] {
        let config_path = temp_home.path().join(config);
        fs::write(&config_path, "# Original content\n")?;
    }

    // Test default initialization
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.arg("init").assert().success();

    // Verify default setup
    let cbp_home = temp_home.path().join(".cbp");
    assert!(cbp_home.exists());
    assert!(cbp_home.join("bin").exists());
    assert!(cbp_home.join("config.toml").exists());

    // Verify shell config updates
    for config in [".bashrc", ".bash_profile", ".zshrc"] {
        let config_path = temp_home.path().join(config);
        let content = fs::read_to_string(&config_path)?;

        // Print content for debugging
        println!("Content of {}:\n{}", config, content);

        assert!(
            content.contains("# .cbp start"),
            "Missing .cbp start marker in {}",
            config
        );
        assert!(content.contains("export PATH=\"$HOME/.cbp/bin:$PATH\""));
        assert!(content.contains("# .cbp end"));
        assert_eq!(content.matches("# .cbp start").count(), 1);
    }

    // Restore original home
    std::env::set_var("HOME", original_home);
    Ok(())
}

#[test]
#[cfg(unix)]
fn command_init_custom_dir() -> anyhow::Result<()> {
    use std::fs;
    use tempfile::TempDir;

    // Create temporary home directory
    let temp_home = TempDir::new()?;
    let original_home = std::env::var("HOME")?;
    std::env::set_var("HOME", temp_home.path());

    // Test custom directory initialization
    let custom_dir = temp_home.path().join("custom_cbp");
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.arg("init").arg(&custom_dir).assert().success();

    // Verify custom directory setup
    let cbp_home = temp_home.path().join(".cbp");
    let config_content = fs::read_to_string(cbp_home.join("config.toml"))?;
    assert!(config_content.contains(&format!("home = \"{}\"", custom_dir.display())));

    // Verify shell config updates
    for config in [".bashrc", ".bash_profile", ".zshrc"] {
        let config_path = temp_home.path().join(config);
        if !config_path.exists() {
            fs::write(&config_path, "# Original content\n")?;
        }

        // Run init again to test idempotency
        Command::cargo_bin("cbp")?
            .arg("init")
            .arg(&custom_dir)
            .assert()
            .success();

        let content = fs::read_to_string(&config_path)?;
        assert!(content.contains("# .cbp start"));
        assert!(content.contains("export PATH=\"$HOME/.cbp/bin:$PATH\""));
        assert!(content.contains(&format!(
            "export PATH=\"{}/bin:$PATH\"",
            custom_dir.display()
        )));
        assert!(content.contains("# .cbp end"));
        assert_eq!(content.matches("# .cbp start").count(), 1);
    }

    // Restore original home
    std::env::set_var("HOME", original_home);
    Ok(())
}

#[test]
#[cfg(windows)]
fn command_init_default() -> anyhow::Result<()> {
    use std::fs;
    use tempfile::TempDir;

    // 创建临时测试目录
    let temp = TempDir::new()?;
    let test_dir = temp.path();

    // 测试默认初始化
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.arg("init")
        .arg("--dir")
        .arg(test_dir)
        .assert()
        .success();

    // 验证目录结构
    let cbp_home = test_dir.join(".cbp");
    assert!(cbp_home.exists());
    assert!(cbp_home.join("bin").exists());
    assert!(cbp_home.join("config.toml").exists());

    // 验证配置文件内容
    let config_content = fs::read_to_string(cbp_home.join("config.toml"))?;
    println!("Config content:\n{}", config_content);
    assert!(config_content.contains("# CBP configuration file"));

    // 验证 PATH 环境变量更新
    let check_output = std::process::Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "[Environment]::GetEnvironmentVariable('Path', \
                [EnvironmentVariableTarget]::User) -split ';' -contains '{}'",
                cbp_home.join("bin").display()
            ),
        ])
        .output()?;

    assert!(
        String::from_utf8_lossy(&check_output.stdout).trim() == "True",
        "PATH environment variable was not updated correctly"
    );

    // 清理环境变量
    std::process::Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "$path = [Environment]::GetEnvironmentVariable('Path', \
                [EnvironmentVariableTarget]::User); \
                $path = ($path -split ';' | Where-Object {{ $_ -ne '{}' }}) -join ';'; \
                [Environment]::SetEnvironmentVariable('Path', $path, \
                [EnvironmentVariableTarget]::User)",
                cbp_home.join("bin").display()
            ),
        ])
        .output()?;

    Ok(())
}
