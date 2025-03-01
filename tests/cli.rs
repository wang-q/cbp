use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn command_invalid() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.arg("foobar");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("recognized"));

    Ok(())
}

#[test]
fn command_kb_readme() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cbp")?;
    let output = cmd.arg("kb").arg("readme").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() > 10);
    assert!(stdout.contains("System Requirements"));

    Ok(())
}

#[test]
fn command_kb_no_args() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.arg("kb");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));

    Ok(())
}

#[test]
fn command_kb_invalid_doc() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.arg("kb").arg("nonexistent");
    cmd.assert().failure();

    Ok(())
}

#[test]
fn command_kb_with_output_file() -> anyhow::Result<()> {
    use std::fs;
    use tempfile::TempDir;

    let temp_dir = TempDir::new()?;
    let test_output = temp_dir.path().join("test_output.md");

    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.arg("kb")
        .arg("readme")
        .arg("-o")
        .arg(&test_output)
        .assert()
        .success();

    let content = fs::read_to_string(&test_output)?;
    assert!(content.contains("System Requirements"));

    Ok(())
}

#[test]
fn test_list_empty() -> anyhow::Result<()> {
    Command::cargo_bin("cbp")?
        .arg("list")
        .arg("--dir")
        .arg("tests/cbp_macos")
        .assert()
        .success()
        .stdout(predicate::str::contains("==> Installed packages:"));

    Ok(())
}

#[test]
fn test_list_packages() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cbp")?;
    let output = cmd
        .arg("list")
        .arg("--dir")
        .arg("tests/cbp_macos")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert!(stdout.contains("zlib"));
    assert!(stdout.contains("bzip2"));

    Ok(())
}

#[test]
fn test_list_specific_package() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cbp")?;
    let output = cmd
        .arg("list")
        .arg("--dir")
        .arg("tests/cbp_macos")
        .arg("zlib")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 10);
    assert!(stdout.contains("include/zlib.h"));
    assert!(stdout.contains("lib/libz.a"));

    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.arg("list")
        .arg("--dir")
        .arg("tests/cbp_macos")
        .arg("nonexistent")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Warning: Package nonexistent is not installed",
        ));

    Ok(())
}

#[test]
fn test_untracted() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("cbp")?;
    let output = cmd
        .arg("untracked")
        .arg("--dir")
        .arg("tests/cbp_macos")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 2);
    assert!(stdout.contains("==> Untracked files"));
    assert!(stdout.contains("\n\n"));

    Ok(())
}
