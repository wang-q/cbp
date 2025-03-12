use assert_cmd::prelude::*;
use std::process::Command;

#[test]
#[cfg(unix)]
fn command_init() -> anyhow::Result<()> {
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
    assert!(custom_dir.exists());
    assert!(custom_dir.join("bin").exists());
    assert!(custom_dir.join("cache").exists());
    assert!(custom_dir.join("records").exists());

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
fn command_init_windows() -> anyhow::Result<()> {
    use tempfile::TempDir;

    // Create temporary test directory
    let temp = TempDir::new()?;
    let test_dir = temp.path();

    // Test default initialization
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.arg("init")
        .arg(test_dir)
        .assert()
        .success();

    // Verify directory structure
    assert!(test_dir.exists());
    assert!(test_dir.join("bin").exists());
    assert!(test_dir.join("cache").exists());
    assert!(test_dir.join("records").exists());

    // Verify PATH environment variable update
    let check_output = std::process::Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "[Environment]::GetEnvironmentVariable('Path', \
                [EnvironmentVariableTarget]::User) -split ';' -contains '{}'",
                test_dir.join("bin").display()
            ),
        ])
        .output()?;

    assert!(
        String::from_utf8_lossy(&check_output.stdout).trim() == "True",
        "PATH environment variable was not updated correctly"
    );

    // Clean up environment variable
    std::process::Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "$path = [Environment]::GetEnvironmentVariable('Path', \
                [EnvironmentVariableTarget]::User); \
                $path = ($path -split ';' | Where-Object {{ $_ -ne '{}' }}) -join ';'; \
                [Environment]::SetEnvironmentVariable('Path', $path, \
                [EnvironmentVariableTarget]::User)",
                test_dir.join("bin").display()
            ),
        ])
        .output()?;

    Ok(())
}
