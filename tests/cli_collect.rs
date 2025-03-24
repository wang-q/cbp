use assert_cmd::prelude::*;
use std::process::Command;
use std::fs;

fn setup_test_files() -> anyhow::Result<tempfile::TempDir> {
    let temp_dir = tempfile::TempDir::new()?;
    
    // Create test files
    fs::write(
        temp_dir.path().join("script.pl"),
        "#!/usr/bin/perl\nprint 'hello';\n",
    )?;
    fs::write(
        temp_dir.path().join("script.py"),
        "#!/usr/bin/python\nprint('hello')\n",
    )?;
    fs::write(temp_dir.path().join("test.txt"), "test content")?;
    
    // Create test directory structure
    fs::create_dir(temp_dir.path().join("bin"))?;
    fs::write(temp_dir.path().join("bin/program"), "binary content")?;
    
    // Create nested directory structure for path testing
    fs::create_dir_all(temp_dir.path().join("doc/sub1"))?;
    fs::create_dir_all(temp_dir.path().join("doc/sub2"))?;
    fs::write(temp_dir.path().join("doc/file1.txt"), "doc1")?;
    fs::write(temp_dir.path().join("doc/sub1/file2.txt"), "doc2")?;
    fs::write(temp_dir.path().join("doc/sub2/file3.txt"), "doc3")?;
    
    Ok(temp_dir)
}

#[test]
fn command_collect_files() -> anyhow::Result<()> {
    let temp_dir = setup_test_files()?;
    let output_tar = temp_dir.path().join("output.tar.gz");

    Command::cargo_bin("cbp")?
        .arg("collect")
        .arg("--mode")
        .arg("files")
        .arg("-o")
        .arg(&output_tar)
        .current_dir(temp_dir.path())  // Set working directory
        .arg("test.txt")  // Use relative path
        .assert()
        .success();

    // Verify archive content
    assert!(output_tar.exists());
    let files = cbp::list_archive_files(&output_tar)?;
    eprintln!("files
     = {:#?}", files
);
    assert!(files.contains("test.txt"));
    Ok(())
}

#[test]
fn command_collect_bin_mode() -> anyhow::Result<()> {
    let temp_dir = setup_test_files()?;
    let output_tar = temp_dir.path().join("output.tar.gz");

    Command::cargo_bin("cbp")?
        .arg("collect")
        .arg("--mode")
        .arg("bin")
        .arg("-o")
        .arg(&output_tar)
        .current_dir(temp_dir.path())  
        .arg("bin/program")
        .assert()
        .success();

    // Verify archive content
    assert!(output_tar.exists());
    let files = cbp::list_archive_files(&output_tar)?;
    eprintln!("files\n    = {:#?}", files);
    assert!(files.contains("bin/program"));
    Ok(())
}

#[test]
fn command_collect_with_shebang() -> anyhow::Result<()> {
    let temp_dir = setup_test_files()?;
    let output_tar = temp_dir.path().join("output.tar.gz");

    Command::cargo_bin("cbp")?
        .arg("collect")
        .arg("--shebang")
        .arg("-o")
        .arg(&output_tar)
        .current_dir(temp_dir.path())  // Set working directory
        .arg("script.pl")  // Use relative path
        .arg("script.py")  // Use relative path
        .assert()
        .success();

    // Verify archive content
    assert!(output_tar.exists());
    let files = cbp::list_archive_files(&output_tar)?;
    assert!(files.contains("script.pl"));
    assert!(files.contains("script.py"));

    // Verify shebang lines are fixed
    let content = cbp::read_file_from_archive(&output_tar, "script.pl")?;
    assert!(content.starts_with("#!/usr/bin/env perl"));

    Ok(())
}

#[test]
fn command_collect_with_copy() -> anyhow::Result<()> {
    let temp_dir = setup_test_files()?;
    let output_tar = temp_dir.path().join("output.tar.gz");

    Command::cargo_bin("cbp")?
        .arg("collect")
        .arg("--copy")
        .arg("test.txt=alias.txt")
        .arg("-o")
        .arg(&output_tar)
        .current_dir(temp_dir.path())  // Set working directory
        .arg("test.txt")  // Use relative path
        .assert()
        .success();

    // Verify archive content
    assert!(output_tar.exists());
    let files = cbp::list_archive_files(&output_tar)?;
    eprintln!("files
    = {:#?}", files
);
    assert!(files.contains("test.txt"));
    assert!(files.contains("alias.txt"));
    Ok(())
}

#[test]
fn command_collect_with_ignore() -> anyhow::Result<()> {
    let temp_dir = setup_test_files()?;
    let output_tar = temp_dir.path().join("output.tar.gz");

    Command::cargo_bin("cbp")?
        .arg("collect")
        .arg("--mode")
        .arg("files")  // 明确指定 files 模式
        .arg("--ignore")
        .arg(".txt")
        .arg("-o")
        .arg(&output_tar)
        .current_dir(temp_dir.path())
        .arg(".")
        .assert()
        .success();

    // Verify archive content
    assert!(output_tar.exists());
    let files = cbp::list_archive_files(&output_tar)?;
    eprintln!("files = {:#?}", files);
    assert!(!files.contains("test.txt")); // Should be ignored
    assert!(files.contains("bin/program")); // Should be included
    Ok(())
}

#[test]
fn command_collect_directory() -> anyhow::Result<()> {
    let temp_dir = setup_test_files()?;
    let output_tar = temp_dir.path().join("output.tar.gz");

    Command::cargo_bin("cbp")?
        .arg("collect")
        .arg("--mode")
        .arg("files")
        .arg("-o")
        .arg(&output_tar)
        .current_dir(temp_dir.path())
        .arg("doc")  // Collect entire directory
        .assert()
        .success();

    // Verify archive content
    assert!(output_tar.exists());
    let files = cbp::list_archive_files(&output_tar)?;
    // eprintln!("files = {:#?}", files);
    
    // Check if all files from the directory are included
    assert!(files.contains("doc/file1.txt"));
    assert!(files.contains("doc/sub1/file2.txt"));
    assert!(files.contains("doc/sub2/file3.txt"));

    Ok(())
}

#[test]
fn command_collect_multiple_paths() -> anyhow::Result<()> {
    let temp_dir = setup_test_files()?;
    let output_tar = temp_dir.path().join("output.tar.gz");

    Command::cargo_bin("cbp")?
        .arg("collect")
        .arg("--mode")
        .arg("files")
        .arg("-o")
        .arg(&output_tar)
        .current_dir(temp_dir.path())
        .arg("doc/sub1")  // First directory
        .arg("doc/file1.txt")  // Single file
        .assert()
        .success();

    // Verify archive content
    assert!(output_tar.exists());
    let files = cbp::list_archive_files(&output_tar)?;
    // eprintln!("files = {:#?}", files);
    
    // Check specific files
    assert!(files.contains("doc/file1.txt"));
    assert!(files.contains("doc/sub1/file2.txt"));
    assert!(!files.contains("doc/sub2/file3.txt")); // Should not be included

    Ok(())
}
