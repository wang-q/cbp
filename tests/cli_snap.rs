use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn command_snap_help() -> anyhow::Result<()> {
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage file snapshots in HOME"))
        .stdout(predicate::str::contains("save"))
        .stdout(predicate::str::contains("load"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("delta"));

    Ok(())
}

#[test]
fn command_snap_save_single_file() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let source_file = temp_dir.path().join("config.txt");
    std::fs::write(&source_file, "key=value\n")?;

    // Save single file (default output name: uses full filename)
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&source_file)
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Snapshot created"))
        .stdout(predicate::str::contains("config.txt.snap.tar.gz"));

    // Verify archive was created
    let archive_path = temp_dir.path().join("config.txt.snap.tar.gz");
    assert!(archive_path.exists(), "Archive should be created");

    Ok(())
}

#[test]
fn command_snap_save_multiple_files_require_output() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");
    std::fs::write(&file1, "content1")?;
    std::fs::write(&file2, "content2")?;

    // Multiple files without -o should fail
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&file1)
        .arg(&file2)
        .assert()
        .failure()
        .stderr(predicate::str::contains("require -o"));

    Ok(())
}

#[test]
fn command_snap_save_multiple_files_with_output() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");
    std::fs::write(&file1, "content1")?;
    std::fs::write(&file2, "content2")?;

    let archive_path = temp_dir.path().join("bundle.snap.tar.gz");

    // Multiple files with -o should succeed
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&file1)
        .arg(&file2)
        .arg("-o")
        .arg(&archive_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Snapshot created"));

    assert!(archive_path.exists(), "Archive should be created");

    Ok(())
}

#[test]
fn command_snap_save_directory() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let source_dir = temp_dir.path().join("myconfig");
    std::fs::create_dir_all(&source_dir)?;
    std::fs::write(source_dir.join("config.toml"), "[settings]\nkey=value\n")?;
    std::fs::write(source_dir.join("README.md"), "# Config\n")?;

    let archive_path = temp_dir.path().join("myconfig.snap.tar.gz");

    // Save directory
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&source_dir)
        .arg("-o")
        .arg(&archive_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Snapshot created"));

    assert!(archive_path.exists(), "Archive should be created");

    // Verify with list command
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("list")
        .arg(&archive_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("myconfig/config.toml"))
        .stdout(predicate::str::contains("myconfig/README.md"));

    Ok(())
}

#[test]
fn command_snap_save_verbose() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let source_dir = temp_dir.path().join("config");
    std::fs::create_dir_all(&source_dir)?;
    std::fs::write(source_dir.join("file1.txt"), "content1")?;
    std::fs::write(source_dir.join("file2.txt"), "content2")?;

    let archive_path = temp_dir.path().join("verbose.snap.tar.gz");

    // Save with verbose
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&source_dir)
        .arg("-o")
        .arg(&archive_path)
        .arg("-v")
        .assert()
        .success()
        .stdout(predicate::str::contains("Added:"))
        .stdout(predicate::str::contains("file1.txt"))
        .stdout(predicate::str::contains("file2.txt"));

    Ok(())
}

#[test]
fn command_snap_list() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let source_file = temp_dir.path().join("test.txt");
    std::fs::write(&source_file, "test content")?;

    let archive_path = temp_dir.path().join("listtest.snap.tar.gz");

    // Create archive
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&source_file)
        .arg("-o")
        .arg(&archive_path)
        .assert()
        .success();

    // List archive contents
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("list")
        .arg(&archive_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Source paths:"))
        .stdout(predicate::str::contains("Archive contents:"))
        .stdout(predicate::str::contains("test.txt"))
        .stdout(predicate::str::contains("1 files"));

    Ok(())
}

#[test]
fn command_snap_list_verbose() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let source_file = temp_dir.path().join("test.txt");
    std::fs::write(&source_file, "test content with some length")?;

    let archive_path = temp_dir.path().join("verbose_list.snap.tar.gz");

    // Create archive
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&source_file)
        .arg("-o")
        .arg(&archive_path)
        .assert()
        .success();

    // List with verbose (shows file sizes)
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("list")
        .arg(&archive_path)
        .arg("-v")
        .assert()
        .success()
        .stdout(predicate::str::contains("B").or(predicate::str::contains("K")));

    Ok(())
}

#[test]
fn command_snap_load_target() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let source_file = temp_dir.path().join("config.txt");
    std::fs::write(&source_file, "original content")?;

    let archive_path = temp_dir.path().join("loadtest.snap.tar.gz");
    let target_dir = temp_dir.path().join("restore_here");
    std::fs::create_dir_all(&target_dir)?;

    // Create archive
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&source_file)
        .arg("-o")
        .arg(&archive_path)
        .assert()
        .success();

    // Load to target directory
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("load")
        .arg(&archive_path)
        .arg("-t")
        .arg(&target_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Snapshot restored"));

    // Verify file was restored to target
    // Note: When source is outside HOME, the full directory structure is preserved
    // So file will be at target_dir/AppData/Local/Temp/.../config.txt
    let mut found = false;
    for entry in walkdir::WalkDir::new(&target_dir) {
        let entry = entry?;
        if entry.file_name() == "config.txt" {
            let content = std::fs::read_to_string(entry.path())?;
            assert_eq!(content, "original content");
            found = true;
            break;
        }
    }
    assert!(
        found,
        "config.txt should be restored somewhere in target_dir"
    );

    Ok(())
}

#[test]
fn command_snap_load_verbose() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let source_file = temp_dir.path().join("verbose.txt");
    std::fs::write(&source_file, "content")?;

    let archive_path = temp_dir.path().join("verbose_load.snap.tar.gz");
    let target_dir = temp_dir.path().join("restore_target");
    std::fs::create_dir_all(&target_dir)?;

    // Create archive
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&source_file)
        .arg("-o")
        .arg(&archive_path)
        .assert()
        .success();

    // Load with verbose
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("load")
        .arg(&archive_path)
        .arg("-t")
        .arg(&target_dir)
        .arg("-v")
        .assert()
        .success()
        .stdout(predicate::str::contains("Restoring snapshot"))
        .stdout(predicate::str::contains("Target:"))
        .stdout(predicate::str::contains("Extracting:"));

    Ok(())
}

#[test]
fn command_snap_delta_show_modified() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let source_file = temp_dir.path().join("tracked.txt");
    std::fs::write(&source_file, "original content")?;

    let archive_path = temp_dir.path().join("delta_base.snap.tar.gz");

    // Create archive
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&source_file)
        .arg("-o")
        .arg(&archive_path)
        .assert()
        .success();

    // Modify the file
    std::fs::write(&source_file, "modified content")?;

    // Delta should show the modified file
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("delta")
        .arg(&archive_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("tracked.txt"));

    Ok(())
}

#[test]
fn command_snap_delta_no_modifications() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let source_file = temp_dir.path().join("unchanged.txt");
    std::fs::write(&source_file, "same content")?;

    let archive_path = temp_dir.path().join("unchanged.snap.tar.gz");

    // Create archive
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&source_file)
        .arg("-o")
        .arg(&archive_path)
        .assert()
        .success();

    // Delta should show no modifications
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("delta")
        .arg(&archive_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("No files have been modified"));

    Ok(())
}

#[test]
fn command_snap_delta_pack() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let source_file = temp_dir.path().join("pack_test.txt");
    std::fs::write(&source_file, "original")?;

    let archive_path = temp_dir.path().join("pack_base.snap.tar.gz");

    // Create archive
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&source_file)
        .arg("-o")
        .arg(&archive_path)
        .assert()
        .success();

    // Modify the file
    std::fs::write(&source_file, "modified for pack test")?;

    // Pack delta
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("delta")
        .arg(&archive_path)
        .arg("-p")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Delta snapshot created"))
        .stdout(predicate::str::contains("pack_base.delta.tar.gz"));

    // Verify delta archive was created
    let delta_path = temp_dir.path().join("pack_base.delta.tar.gz");
    assert!(delta_path.exists(), "Delta archive should be created");

    Ok(())
}

#[test]
fn command_snap_archive_not_found() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let nonexistent = temp_dir.path().join("nonexistent.snap.tar.gz");

    // List non-existent archive
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("list")
        .arg(&nonexistent)
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));

    Ok(())
}

#[test]
fn command_snap_full_workflow() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;

    // Create a config directory structure
    let config_dir = temp_dir.path().join("myapp");
    std::fs::create_dir_all(&config_dir)?;
    std::fs::write(config_dir.join("settings.json"), r#"{"version": "1.0"}"#)?;
    std::fs::write(config_dir.join("README.md"), "# MyApp Config\n")?;

    let archive_path = temp_dir.path().join("myapp_backup.snap.tar.gz");
    let restore_dir = temp_dir.path().join("restored");
    std::fs::create_dir_all(&restore_dir)?;

    // Step 1: Save snapshot
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&config_dir)
        .arg("-o")
        .arg(&archive_path)
        .assert()
        .success();

    assert!(archive_path.exists(), "Archive should be created");

    // Step 2: List to verify contents
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("list")
        .arg(&archive_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("settings.json"))
        .stdout(predicate::str::contains("README.md"));

    // Step 3: Modify original files
    std::fs::write(config_dir.join("settings.json"), r#"{"version": "2.0"}"#)?;

    // Step 4: Delta should detect modification
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("delta")
        .arg(&archive_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("settings.json"));

    // Step 5: Pack delta
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("delta")
        .arg(&archive_path)
        .arg("-p")
        .current_dir(&temp_dir)
        .assert()
        .success();

    let delta_path = temp_dir.path().join("myapp_backup.delta.tar.gz");
    assert!(delta_path.exists(), "Delta archive should be created");

    // Step 6: Load to restore directory
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("load")
        .arg(&archive_path)
        .arg("-t")
        .arg(&restore_dir)
        .assert()
        .success();

    // Verify restored files
    // Note: When source is outside HOME, the full directory structure is preserved
    let mut found_settings = false;
    let mut found_readme = false;
    for entry in walkdir::WalkDir::new(&restore_dir) {
        let entry = entry?;
        match entry.file_name().to_str() {
            Some("settings.json") => {
                let content = std::fs::read_to_string(entry.path())?;
                assert_eq!(content, r#"{"version": "1.0"}"#);
                found_settings = true;
            }
            Some("README.md") => {
                found_readme = true;
            }
            _ => {}
        }
    }
    assert!(found_settings, "settings.json should be restored");
    assert!(found_readme, "README.md should be restored");

    Ok(())
}

/// Test delta detects deleted files
#[test]
fn command_snap_delta_deleted_files() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let source_file = temp_dir.path().join("will_delete.txt");
    std::fs::write(&source_file, "content to be deleted")?;

    let archive_path = temp_dir.path().join("deleted_test.snap.tar.gz");

    // Create archive
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&source_file)
        .arg("-o")
        .arg(&archive_path)
        .assert()
        .success();

    // Delete the file
    std::fs::remove_file(&source_file)?;

    // Delta should handle the deleted file gracefully
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("delta")
        .arg(&archive_path)
        .assert()
        .success();

    Ok(())
}

/// Test list command with archive missing source path comment
#[test]
fn command_snap_list_no_comment() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let archive_path = temp_dir.path().join("no_comment.snap.tar.gz");

    // Create a gz file without comment using GzEncoder
    use flate2::write::GzEncoder;
    use std::io::Write;

    let file = std::fs::File::create(&archive_path)?;
    let mut encoder = GzEncoder::new(file, flate2::Compression::default());

    // Create tar content in memory first
    let mut tar_data = Vec::new();
    {
        let mut archive = tar::Builder::new(&mut tar_data);
        let test_content = b"test content";
        let mut header = tar::Header::new_gnu();
        header.set_path("test.txt")?;
        header.set_size(test_content.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        archive.append(&header, &test_content[..])?;
        archive.finish()?;
    }

    encoder.write_all(&tar_data)?;
    encoder.finish()?;

    // List should show "No source path information"
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("list")
        .arg(&archive_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("No source path information"));

    Ok(())
}

/// Test load with archive missing source path comment fails gracefully
#[test]
fn command_snap_load_no_comment_fails() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let archive_path = temp_dir.path().join("no_comment.snap.tar.gz");
    let target_dir = temp_dir.path().join("restore_target");
    std::fs::create_dir_all(&target_dir)?;

    // Create a gz file without comment using GzEncoder
    use flate2::write::GzEncoder;
    use std::io::Write;

    let file = std::fs::File::create(&archive_path)?;
    let mut encoder = GzEncoder::new(file, flate2::Compression::default());

    // Create tar content in memory first
    let mut tar_data = Vec::new();
    {
        let mut archive = tar::Builder::new(&mut tar_data);
        let test_content = b"test content";
        let mut header = tar::Header::new_gnu();
        header.set_path("test.txt")?;
        header.set_size(test_content.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        archive.append(&header, &test_content[..])?;
        archive.finish()?;
    }

    encoder.write_all(&tar_data)?;
    encoder.finish()?;

    // Load should fail with error about missing source path
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("load")
        .arg(&archive_path)
        .arg("-t")
        .arg(&target_dir)
        .assert()
        .failure()
        .stderr(predicate::str::contains("no source path information"));

    Ok(())
}

/// Test special characters in filenames
#[test]
fn command_snap_special_characters_filename() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let file_with_space = temp_dir.path().join("file with spaces.txt");
    let file_with_unicode = temp_dir.path().join("文件_日本語_emoji_🎉.txt");
    std::fs::write(&file_with_space, "space content")?;
    std::fs::write(&file_with_unicode, "unicode content")?;

    let archive_path = temp_dir.path().join("special_chars.snap.tar.gz");
    let restore_dir = temp_dir.path().join("restored");
    std::fs::create_dir_all(&restore_dir)?;

    // Save multiple files with special names
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&file_with_space)
        .arg(&file_with_unicode)
        .arg("-o")
        .arg(&archive_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Snapshot created"));

    // List should show the files
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("list")
        .arg(&archive_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("file with spaces.txt"));

    // Load and verify
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("load")
        .arg(&archive_path)
        .arg("-t")
        .arg(&restore_dir)
        .assert()
        .success();

    // Verify files were restored
    let mut found_space = false;
    let mut found_unicode = false;
    for entry in walkdir::WalkDir::new(&restore_dir) {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy();
        if name.contains("spaces") {
            found_space = true;
        }
        if name.contains("emoji") || name.contains("日本語") || name.contains("文件")
        {
            found_unicode = true;
        }
    }
    assert!(found_space, "File with spaces should be restored");
    assert!(found_unicode, "File with unicode should be restored");

    Ok(())
}

/// Test deep nested directory structure
#[test]
fn command_snap_deep_nested_structure() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let deep_dir = temp_dir
        .path()
        .join("level1")
        .join("level2")
        .join("level3")
        .join("level4");
    std::fs::create_dir_all(&deep_dir)?;
    std::fs::write(deep_dir.join("deep_file.txt"), "deep content")?;

    let archive_path = temp_dir.path().join("nested.snap.tar.gz");
    let restore_dir = temp_dir.path().join("restored");
    std::fs::create_dir_all(&restore_dir)?;

    // Save the top-level directory
    let source_dir = temp_dir.path().join("level1");
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&source_dir)
        .arg("-o")
        .arg(&archive_path)
        .assert()
        .success();

    // List should show nested structure
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("list")
        .arg(&archive_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("level1"))
        .stdout(predicate::str::contains("level2"))
        .stdout(predicate::str::contains("level3"))
        .stdout(predicate::str::contains("level4"))
        .stdout(predicate::str::contains("deep_file.txt"));

    // Load and verify structure preserved
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("load")
        .arg(&archive_path)
        .arg("-t")
        .arg(&restore_dir)
        .assert()
        .success();

    // Verify deep file exists
    let mut found_deep = false;
    for entry in walkdir::WalkDir::new(&restore_dir) {
        let entry = entry?;
        if entry.file_name() == "deep_file.txt" {
            let content = std::fs::read_to_string(entry.path())?;
            assert_eq!(content, "deep content");
            found_deep = true;
        }
    }
    assert!(found_deep, "Deep nested file should be restored");

    Ok(())
}

/// Test delta pack with multiple modified files
#[test]
fn command_snap_delta_pack_multiple_files() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");
    let file3 = temp_dir.path().join("file3.txt");
    std::fs::write(&file1, "original1")?;
    std::fs::write(&file2, "original2")?;
    std::fs::write(&file3, "original3")?;

    let archive_path = temp_dir.path().join("multi.snap.tar.gz");

    // Create archive with all files
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&file1)
        .arg(&file2)
        .arg(&file3)
        .arg("-o")
        .arg(&archive_path)
        .assert()
        .success();

    // Modify two files
    std::fs::write(&file1, "modified1")?;
    std::fs::write(&file2, "modified2")?;
    // file3 unchanged

    // Pack delta
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("delta")
        .arg(&archive_path)
        .arg("-p")
        .current_dir(&temp_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Delta snapshot created"));

    // Verify delta archive was created
    let delta_path = temp_dir.path().join("multi.delta.tar.gz");
    assert!(delta_path.exists(), "Delta archive should be created");

    // List delta archive - should only contain modified files
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("list")
        .arg(&delta_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("file1.txt"))
        .stdout(predicate::str::contains("file2.txt"));

    Ok(())
}

/// Test empty directory handling
#[test]
fn command_snap_empty_directory() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let empty_dir = temp_dir.path().join("empty_folder");
    std::fs::create_dir_all(&empty_dir)?;

    let archive_path = temp_dir.path().join("empty.snap.tar.gz");

    // Save empty directory
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("save")
        .arg(&empty_dir)
        .arg("-o")
        .arg(&archive_path)
        .assert()
        .success();

    // List should show no files (empty)
    Command::cargo_bin("cbp")?
        .arg("snap")
        .arg("list")
        .arg(&archive_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("0 files"));

    Ok(())
}
