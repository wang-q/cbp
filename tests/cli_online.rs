use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn command_avail() -> anyhow::Result<()> {
    // Create mock server
    let mut server = mockito::Server::new();

    // Prepare mock JSON response
    let mock_response = r#"{
        "assets": [
            {"name": "zlib.macos.tar.gz"},
            {"name": "bzip2.macos.tar.gz"},
            {"name": "zlib.linux.tar.gz"},
            {"name": "bzip2.linux.tar.gz"}
        ]
    }"#;

    // Set up mock endpoint
    let _m = server
        .mock("GET", "/repos/wang-q/cbp/releases/tags/Binaries")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response)
        .create();

    // Override GitHub API URL with environment variable
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.env("GITHUB_API_URL", &server.url())
        .arg("avail")
        .arg("macos");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("zlib"))
        .stdout(predicate::str::contains("bzip2"));

    Ok(())
}

#[test]
fn command_install() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;

    // Create mock server
    let mut server = mockito::Server::new();

    // Prepare test package data
    let test_package = include_bytes!("zlib.macos.tar.gz");

    // Set up mock endpoints for different platforms using the same test package
    let _m1 = server
        .mock(
            "GET",
            "/wang-q/cbp/releases/download/Binaries/zlib.macos.tar.gz",
        )
        .with_status(200)
        .with_header("content-type", "application/gzip")
        .with_body(test_package)
        .create();

    let _m2 = server
        .mock(
            "GET",
            "/wang-q/cbp/releases/download/Binaries/zlib.linux.tar.gz",
        )
        .with_status(200)
        .with_header("content-type", "application/gzip")
        .with_body(test_package)
        .create();

    let _m3 = server
        .mock(
            "GET",
            "/wang-q/cbp/releases/download/Binaries/zlib.windows.tar.gz",
        )
        .with_status(200)
        .with_header("content-type", "application/gzip")
        .with_body(test_package)
        .create();

    // Override GitHub URL with environment variable
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.env("GITHUB_RELEASE_URL", &server.url())
        .arg("install")
        .arg("--dir")
        .arg(temp_dir.path())
        .arg("zlib");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("==> Downloading zlib"))
        .stdout(predicate::str::contains("Done"));

    // Verify installation results
    assert!(temp_dir.path().join("include/zlib.h").exists());
    assert!(temp_dir.path().join("lib/libz.a").exists());
    assert!(temp_dir.path().join("records/zlib.files").exists());

    Ok(())
}

#[test]
fn command_info() -> anyhow::Result<()> {
    // Create mock server
    let mut server = mockito::Server::new();

    // Prepare mock JSON response
    let mock_response = r#"{
        "name": "newick-utils",
        "version": "1.6",
        "description": "A suite of utilities for processing phylogenetic trees",
        "homepage": "http://cegg.unige.ch/newick_utils",
        "license": "BSD-3-Clause",
        "dependencies": ["zlib", "readline"]
    }"#;

    // Set up mock endpoint
    let _m = server
        .mock("GET", "/wang-q/cbp/master/packages/newick-utils.json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response)
        .create();

    // Test normal output
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.env("GITHUB_RAW_URL", &server.url())
        .arg("info")
        .arg("newick-utils");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("==> Package info: newick-utils"))
        .stdout(predicate::str::contains("Version: 1.6"))
        .stdout(predicate::str::contains("License: BSD-3-Clause"))
        .stdout(predicate::str::contains("- zlib"))
        .stdout(predicate::str::contains("- readline"));

    // Test JSON output
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.env("GITHUB_RAW_URL", &server.url())
        .arg("info")
        .arg("newick-utils")
        .arg("--json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"name\": \"newick-utils\""))
        .stdout(predicate::str::contains("\"version\": \"1.6\""));

    // Test non-existent package
    let _m = server
        .mock("GET", "/wang-q/cbp/master/packages/non-existent.json")
        .with_status(404)
        .create();

    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.env("GITHUB_RAW_URL", &server.url())
        .arg("info")
        .arg("non-existent");

    cmd.assert().failure();

    Ok(())
}

#[test]
#[cfg_attr(
    target_os = "windows",
    ignore = "tar operations not supported on Windows"
)]
fn command_build_source() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;

    // Create mock server
    let mut server = mockito::Server::new();

    // Prepare test package data
    let test_package = include_bytes!("TRF-4.09.1.tar.gz");

    // Set up mock endpoints
    let _m1 = server
        .mock(
            "GET",
            "/Benson-Genomics-Lab/TRF/archive/refs/tags/v4.09.1.tar.gz",
        )
        .with_status(200)
        .with_header("content-type", "application/gzip")
        .with_body(test_package)
        .create();

    // Create package directory and copy the existing package JSON
    std::fs::create_dir_all(temp_dir.path().join("packages"))?;
    let cargo_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    std::fs::copy(
        std::path::Path::new(&cargo_dir).join("packages/trf.json"),
        temp_dir.path().join("packages/trf.json"),
    )?;

    // Run download command
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.env("GITHUB_RELEASE_URL", &server.url())
        .arg("build")
        .arg("source")
        .arg("--dir")
        .arg(temp_dir.path())
        .arg("trf");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("==> Processing source package: trf"))
        .stdout(predicate::str::contains("-> Downloading from"))
        .stdout(predicate::str::contains("-> Processing source archive"))
        .stdout(predicate::str::contains(
            "-> Successfully downloaded and processed",
        ));

    // Debug: Print directory structure
    eprintln!("==> Temporary directory structure:");
    for entry in walkdir::WalkDir::new(temp_dir.path()) {
        let entry = entry?;
        let path = entry.path().strip_prefix(temp_dir.path())?;
        eprintln!("  {}", path.display());
    }

    // Verify download results
    let output_tar = temp_dir.path().join("sources/trf.tar.gz");
    assert!(output_tar.exists());

    // Debug: Print archive contents
    eprintln!("\n==> Archive contents:");
    let files = cbp::list_archive_files(&output_tar)?;
    eprintln!("  {}", files);

    // Perform checks
    assert!(files.contains("trf/"));
    assert!(files.contains("trf/INSTALL"));
    assert!(!files.contains("TRF-4.09.1/")); // Should be renamed
    assert!(!files.contains("config.h.in~")); // Should be cleaned

    Ok(())
}

#[test]
#[cfg_attr(
    target_os = "windows",
    ignore = "tar operations not supported on Windows"
)]
fn command_build_font() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;

    // Create mock server
    let mut server = mockito::Server::new();

    // Prepare test package data
    let test_package = include_bytes!("Charter 210112.zip");

    // Set up mock endpoint
    let _m = server
        .mock("GET", "/practicaltypography.com/fonts/Charter%20210112.zip")
        .with_status(200)
        .with_header("content-type", "application/zip")
        .with_body(test_package)
        .create();

    // Create package directory and copy the existing package JSON
    std::fs::create_dir_all(temp_dir.path().join("packages"))?;
    let cargo_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    std::fs::copy(
        std::path::Path::new(&cargo_dir).join("packages/charter.json"),
        temp_dir.path().join("packages/charter.json"),
    )?;

    // Run font command
    let mut cmd = Command::cargo_bin("cbp")?;
    cmd.env("GITHUB_RELEASE_URL", &server.url())
        .arg("build")
        .arg("font")
        .arg("--dir")
        .arg(temp_dir.path())
        .arg("charter");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "==> Processing font package: charter",
        ))
        .stdout(predicate::str::contains("-> Downloading from"))
        .stdout(predicate::str::contains(
            "-> Font package created successfully",
        ));

    // Verify build results
    let output_tar = temp_dir.path().join("binaries/charter.font.tar.gz");
    assert!(output_tar.exists());

    // Debug: Print archive contents
    eprintln!("\n==> Archive contents:");
    let files = cbp::list_archive_files(&output_tar)?;
    eprintln!("  {}", files);

    // Perform checks
    assert!(files.contains("Charter Regular.ttf"));
    assert!(!files.contains("__MACOSX")); // Should be cleaned

    Ok(())
}
