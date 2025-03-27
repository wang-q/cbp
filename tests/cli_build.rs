use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn command_build_validate() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;

    // Create test directory structure
    std::fs::create_dir_all(temp_dir.path().join("packages"))?;
    let cargo_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
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
        .arg("--base")
        .arg(temp_dir.path())
        .arg("zlib")
        .assert()
        .success()
        .stdout(predicate::str::contains("Validating zlib... PASSED"));

    // Test validating an invalid package
    Command::cargo_bin("cbp")?
        .arg("build")
        .arg("validate")
        .arg("--base")
        .arg(temp_dir.path())
        .arg("invalid")
        .assert()
        .failure()
        .stdout(predicate::str::contains("Validating invalid... FAILED"));

    // Test validating a non-existent package
    Command::cargo_bin("cbp")?
        .arg("build")
        .arg("validate")
        .arg("--base")
        .arg(temp_dir.path())
        .arg("nonexistent")
        .assert()
        .failure()
        .stdout(predicate::str::contains("Package file does not exist"));

    Ok(())
}

fn setup_test_data() -> anyhow::Result<tempfile::TempDir> {
    let temp_dir = tempfile::TempDir::new()?;

    // Extract test data
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(
        std::fs::File::open("tests/cbp_macos.tar.gz")?,
    ));
    archive.unpack(temp_dir.path())?;

    Ok(temp_dir)
}

#[test]
fn command_build_test_font() -> anyhow::Result<()> {
    let temp_dir = setup_test_data()?;

    // Create cache directory and copy test package
    let cbp_home = temp_dir.path();
    std::fs::create_dir_all(cbp_home.join("cache"))?;
    let cargo_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    std::fs::copy(
        std::path::Path::new(&cargo_dir).join("tests/charter.font.tar.gz"),
        cbp_home.join("cache").join("charter.font.tar.gz"),
    )?;

    // Run local command with --dir option
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.arg("local")
        .arg("--dir")
        .arg(cbp_home)
        .arg("--type")
        .arg("font")
        .arg("charter")
        .current_dir(cbp_home);
    cmd.assert().success();

    // Create test directory structure
    std::fs::create_dir_all(cbp_home.join("packages"))?;
    let cargo_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    std::fs::copy(
        std::path::Path::new(&cargo_dir).join("packages/charter.json"),
        temp_dir.path().join("packages/charter.json"),
    )?;

    // Run test command
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.arg("build")
        .arg("test")
        .arg("--dir")
        .arg(cbp_home)
        .arg("--base")
        .arg(cbp_home)
        .arg("charter")
        .assert()
        .success()
        .stdout(predicate::str::contains("==> Testing package: charter"))
        .stdout(predicate::str::contains("PASSED"));

    Ok(())
}
