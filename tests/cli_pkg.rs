use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

fn setup_test_data() -> anyhow::Result<tempfile::TempDir> {
    let temp_dir = tempfile::TempDir::new()?;

    // Extract test data
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(
        std::fs::File::open("tests/cbp_macos.tar.gz")?,
    ));
    archive.unpack(temp_dir.path())?;

    Ok(temp_dir)
}

fn list_dir_contents(dir: &std::path::Path, level: usize) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        println!(
            "{:indent$}{}",
            "",
            path.strip_prefix(dir)?.display(),
            indent = level * 2
        );
        if path.is_dir() {
            list_dir_contents(&path, level + 1)?;
        }
    }
    Ok(())
}

#[test]
fn command_list_empty() -> anyhow::Result<()> {
    let temp_dir = setup_test_data()?;
    Command::cargo_bin("cbp")?
        .arg("list")
        .arg("--dir")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("==> Installed packages:"));

    Ok(())
}

#[test]
fn command_list_packages() -> anyhow::Result<()> {
    let temp_dir = setup_test_data()?;
    list_dir_contents(temp_dir.path(), 0)?;

    let mut cmd = Command::cargo_bin("cbp")?;
    let output = cmd
        .arg("list")
        .arg("--dir")
        .arg(temp_dir.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    print!("{}", stdout);

    assert_eq!(stdout.lines().count(), 4);
    assert!(stdout.contains("zlib"));
    assert!(stdout.contains("bzip2"));

    Ok(())
}

#[test]
fn command_list_specific_package() -> anyhow::Result<()> {
    let temp_dir = setup_test_data()?;

    let mut cmd = Command::cargo_bin("cbp")?;
    let output = cmd
        .arg("list")
        .arg("--dir")
        .arg(temp_dir.path())
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
        .arg(temp_dir.path())
        .arg("nonexistent")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "==> Package nonexistent is not installed",
        ));

    Ok(())
}

#[test]
fn command_check() -> anyhow::Result<()> {
    let temp_dir = setup_test_data()?;
    let mut cmd = Command::cargo_bin("cbp")?;
    let output = cmd
        .arg("check")
        .arg("--dir")
        .arg(temp_dir.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() > 0);
    assert!(stdout.contains("==> Unmanaged files"));

    Ok(())
}

#[test]
fn command_remove() -> anyhow::Result<()> {
    let temp_dir = setup_test_data()?;
    let dest_dir = temp_dir.path();

    // Test removing non-existent package
    Command::cargo_bin("cbp")?
        .arg("remove")
        .arg("--dir")
        .arg(&dest_dir)
        .arg("nonexistent")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "==> Package nonexistent is not installed",
        ));

    // Test removing existing package
    Command::cargo_bin("cbp")?
        .arg("remove")
        .arg("--dir")
        .arg(&dest_dir)
        .arg("zlib")
        .assert()
        .success()
        .stdout(predicate::str::contains("==> Removing zlib"))
        .stdout(predicate::str::contains("Done"));

    // Verify package is removed
    assert!(!dest_dir.join("records/zlib.files").exists());
    assert!(!dest_dir.join("lib/zlib.a").exists());

    // Verify other package still exists
    assert!(dest_dir.join("records/bzip2.files").exists());
    assert!(dest_dir.join("lib/libbz2.a").exists());

    Ok(())
}

#[test]
fn command_local() -> anyhow::Result<()> {
    let temp_dir = setup_test_data()?;
    let temp = tempfile::TempDir::new()?;

    // Create cache directory and copy test package
    let cbp_home = temp.path();
    std::fs::create_dir_all(cbp_home.join("cache"))?;
    let os_type = cbp::get_os_type()?;
    let pkg_file = format!("zlib.{}.tar.gz", os_type);
    std::fs::copy(
        temp_dir.path().join("cache/zlib.macos.tar.gz"),
        cbp_home.join("cache").join(&pkg_file),
    )?;

    // Run local command with --dir option
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.arg("local")
        .arg("--dir")
        .arg(cbp_home)
        .arg("zlib")
        .current_dir(temp.path());
    cmd.assert().success();

    // Verify installation
    assert!(cbp_home.join("include/zlib.h").exists());
    assert!(cbp_home.join("lib/libz.a").exists());
    assert!(cbp_home.join("records/zlib.files").exists());

    Ok(())
}
