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
    assert!(stdout.contains("bioinformatics tools"));

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
    assert!(content.contains("bioinformatics tools"));

    Ok(())
}

fn setup_test_data() -> anyhow::Result<tempfile::TempDir> {
    let temp_dir = tempfile::TempDir::new()?;

    // 解压测试数据
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(
        std::fs::File::open("tests/cbp_macos.tar.gz")?,
    ));
    archive.unpack(temp_dir.path())?;

    Ok(temp_dir)
}

#[test]
fn command_list_empty() -> anyhow::Result<()> {
    let temp_dir = setup_test_data()?;
    Command::cargo_bin("cbp")?
        .arg("list")
        .arg("--dir")
        .arg(temp_dir.path().join("cbp_macos"))
        .assert()
        .success()
        .stdout(predicate::str::contains("==> Installed packages:"));

    Ok(())
}

#[test]
fn command_list_packages() -> anyhow::Result<()> {
    let temp_dir = setup_test_data()?;
    let mut cmd = Command::cargo_bin("cbp")?;
    let output = cmd
        .arg("list")
        .arg("--dir")
        .arg(temp_dir.path().join("cbp_macos"))
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
        .arg(temp_dir.path().join("cbp_macos"))
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
        .arg(temp_dir.path().join("cbp_macos"))
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
    let dest_dir = temp_dir.path().join("cbp_macos");

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

    // Set up CBP_HOME
    std::env::set_var("HOME", temp.path());
    let cbp_home = temp.path().join(".cbp");

    // Create cache directory and copy test package
    std::fs::create_dir_all(cbp_home.join("cache"))?;
    let os_type = cbp::get_os_type()?;
    let pkg_file = format!("zlib.{}.tar.gz", os_type);
    std::fs::copy(
        temp_dir.path().join("cbp_macos/cache/zlib.macos.tar.gz"),
        cbp_home.join("cache").join(&pkg_file),
    )?;

    // Run local command
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.arg("local").arg("zlib").current_dir(temp.path());
    cmd.assert().success();

    // Verify installation
    assert!(cbp_home.join("include/zlib.h").exists());
    assert!(cbp_home.join("lib/libz.a").exists());
    assert!(cbp_home.join("records/zlib.files").exists());

    Ok(())
}

#[test]
fn command_tar() -> anyhow::Result<()> {
    use std::fs;
    use tempfile::TempDir;

    // Create test directory
    let temp_dir = TempDir::new()?;
    let collect_dir = temp_dir.path().join("collect");
    fs::create_dir(&collect_dir)?;

    // Create test files
    fs::write(collect_dir.join("test.txt"), "test content")?;
    fs::create_dir_all(collect_dir.join("lib"))?;
    fs::write(collect_dir.join("lib/libtest.a"), "test lib")?;

    // Create system files (should be filtered)
    fs::write(collect_dir.join(".DS_Store"), "system file")?;
    fs::write(collect_dir.join("._test"), "resource fork")?;

    // Create doc directory (should be removed)
    fs::create_dir_all(collect_dir.join("share/man"))?;
    fs::write(collect_dir.join("share/man/test.1"), "man page")?;

    // Run tar command
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.arg("tar")
        .arg("-o")
        .arg(format!("test.{}.tar.gz", cbp::get_os_type()?))
        .arg(&collect_dir)
        .arg("--cleanup") // Add cleanup flag
        .current_dir(temp_dir.path())
        .assert()
        .success();

    // Verify package file
    let tar_name = format!("test.{}.tar.gz", cbp::get_os_type()?);
    assert!(temp_dir.path().join(&tar_name).exists());

    // Verify package contents
    let tar_file = fs::File::open(temp_dir.path().join(&tar_name))?;
    let gz = flate2::read::GzDecoder::new(tar_file);
    let mut archive = tar::Archive::new(gz);
    let entries: Vec<_> = archive.entries()?.collect::<Result<_, _>>()?;

    // Verify file list
    let paths: Vec<_> = entries
        .iter()
        .map(|e| e.path().unwrap().to_string_lossy().into_owned())
        .collect();

    assert!(paths.contains(&"test.txt".to_string()));
    assert!(paths.contains(&"lib/libtest.a".to_string()));
    assert!(!paths.contains(&".DS_Store".to_string()));
    assert!(!paths.contains(&"._test".to_string()));
    assert!(!paths.contains(&"share/man/test.1".to_string()));

    Ok(())
}

#[test]
#[cfg(unix)]
fn command_tar_symlink() -> anyhow::Result<()> {
    use std::fs;
    use std::os::unix::fs::symlink;
    use tempfile::TempDir;

    // Create test directory
    let temp_dir = TempDir::new()?;
    let collect_dir = temp_dir.path().join("collect");
    fs::create_dir(&collect_dir)?;

    // Create test files and symlink
    fs::write(collect_dir.join("test.txt"), "test content")?;
    std::env::set_current_dir(&collect_dir)?;
    symlink("test.txt", "test.link")?;
    
    // Verify symlink was created correctly
    let link_path = collect_dir.join("test.link");
    println!("Symlink exists: {}", link_path.exists());
    println!("Is symlink: {}", link_path.is_symlink());
    if link_path.is_symlink() {
        println!("Symlink target: {:?}", fs::read_link(&link_path)?);
    }
    
    std::env::set_current_dir(temp_dir.path())?;

    // Run tar command
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.arg("tar")
        .arg("-o")
        .arg(format!("test.{}.tar.gz", cbp::get_os_type()?))
        .arg(&collect_dir)
        .current_dir(temp_dir.path())
        .assert()
        .success();

    // Verify package contents
    let tar_file = fs::File::open(temp_dir.path().join(format!("test.{}.tar.gz", cbp::get_os_type()?)))?;
    let gz = flate2::read::GzDecoder::new(tar_file);
    let mut archive = tar::Archive::new(gz);
    let entries: Vec<_> = archive.entries()?.collect::<Result<_, _>>()?;

    // Verify file list with symlink
    let paths: Vec<_> = entries
        .iter()
        .map(|e| {
            let path = e.path().unwrap().to_string_lossy().into_owned();
            println!("Entry type: {:?}", e.header().entry_type());
            if e.header().entry_type() == tar::EntryType::Symlink {
                let link_name = e.link_name().unwrap().unwrap();
                let link_target = link_name.to_string_lossy().into_owned();
                println!("Found symlink: {} -> {}", path, link_target);
                format!("{} -> {}", path, link_target)
            } else {
                println!("Found file: {}", path);
                path
            }
        })
        .collect();

    println!("\nAll paths:");
    for path in &paths {
        println!("  {}", path);
    }

    assert!(paths.contains(&"test.txt".to_string()));
    assert!(paths.contains(&"test.link -> test.txt".to_string()));

    Ok(())
}
