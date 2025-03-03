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
    let tar_file = fs::File::open(
        temp_dir
            .path()
            .join(format!("test.{}.tar.gz", cbp::get_os_type()?)),
    )?;
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

#[test]
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
    cmd.arg("init")
        .arg("--dir")
        .arg(&custom_dir)
        .assert()
        .success();

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
            .arg("--dir")
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
fn command_prefix() -> anyhow::Result<()> {
    use tempfile::TempDir;

    let temp = TempDir::new()?;
    let cbp_home = temp.path();

    // Test default behavior (no args)
    let output = Command::cargo_bin("cbp")?
        .arg("prefix")
        .arg("--dir")
        .arg(cbp_home)
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;
    assert_eq!(stdout.trim(), cbp_home.to_string_lossy().trim());

    // 创建所有路径的字符串表示
    let bin_path = cbp_home.join("bin").to_string_lossy().into_owned();
    let cache_path = cbp_home.join("cache").to_string_lossy().into_owned();
    let records_path = cbp_home.join("records").to_string_lossy().into_owned();
    let config_path = cbp_home.to_string_lossy().into_owned();
    let include_path = cbp_home.join("include").to_string_lossy().into_owned();
    let lib_path = cbp_home.join("lib").to_string_lossy().into_owned();
    let exe_path = cbp_home.join("bin/cbp").to_string_lossy().into_owned();

    // Test all directory options
    let test_cases = [
        ("bin", bin_path.as_str()),
        ("cache", cache_path.as_str()),
        ("records", records_path.as_str()),
        ("config", config_path.as_str()),
        ("include", include_path.as_str()),
        ("lib", lib_path.as_str()),
        ("exe", exe_path.as_str()),
    ];

    for (dir_type, expected_path) in test_cases {
        let output = Command::cargo_bin("cbp")?
            .arg("prefix")
            .arg("--dir")
            .arg(cbp_home)
            .arg(dir_type)
            .output()?;
        let stdout = String::from_utf8(output.stdout)?;
        assert_eq!(
            stdout.trim(),
            expected_path,
            "Failed for directory type: {}",
            dir_type
        );
    }

    // Test invalid directory type
    Command::cargo_bin("cbp")?
        .arg("prefix")
        .arg("--dir")
        .arg(cbp_home)
        .arg("invalid")
        .assert()
        .failure();

    Ok(())
}
