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
