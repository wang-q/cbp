use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn command_build_validate() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;

    // Create test directory structure
    std::fs::create_dir_all(temp_dir.path().join("packages"))?;
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    std::fs::copy(
        std::path::Path::new(&cargo_dir).join("packages/zlib.json"),
        temp_dir.path().join("packages/zlib.json"),
    )?;

    // Create an invalid package configuration file
    std::fs::write(
        temp_dir.path().join("packages/invalid.json"),
        r#"{
            "name": "invalid"
        }"#,
    )?;

    // Test validating a valid package
    Command::cargo_bin("cbp")?
        .arg("build")
        .arg("validate")
        .arg("--dir")
        .arg(temp_dir.path())
        .arg("zlib")
        .assert()
        .success()
        .stdout(predicate::str::contains("Validating zlib... PASSED"));

    // Test validating an invalid package
    Command::cargo_bin("cbp")?
        .arg("build")
        .arg("validate")
        .arg("--dir")
        .arg(temp_dir.path())
        .arg("invalid")
        .assert()
        .failure()
        .stdout(predicate::str::contains("Validating invalid... FAILED"));

    // Test validating a non-existent package
    Command::cargo_bin("cbp")?
        .arg("build")
        .arg("validate")
        .arg("--dir")
        .arg(temp_dir.path())
        .arg("nonexistent")
        .assert()
        .failure()
        .stdout(predicate::str::contains("Package file does not exist"));

    Ok(())
}
