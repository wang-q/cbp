use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn command_avail() -> anyhow::Result<()> {
    // 创建模拟服务器
    let mut server = mockito::Server::new();

    // 准备模拟的 JSON 响应
    let mock_response = r#"{
        "assets": [
            {"name": "zlib.macos.tar.gz"},
            {"name": "bzip2.macos.tar.gz"},
            {"name": "zlib.linux.tar.gz"},
            {"name": "bzip2.linux.tar.gz"}
        ]
    }"#;

    // 设置模拟端点
    let _m = server.mock("GET", "/repos/wang-q/cbp/releases/tags/Binaries")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response)
        .create();

    // 使用环境变量临时覆盖 GitHub API URL
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
    
    // 创建模拟服务器
    let mut server = mockito::Server::new();
    
    // 准备测试包数据
    let test_package = include_bytes!("zlib.macos.tar.gz");
    
    // 设置模拟端点
    // 设置多个模拟端点，使用同一个测试包
    let _m1 = server.mock("GET", "/wang-q/cbp/releases/download/Binaries/zlib.macos.tar.gz")
        .with_status(200)
        .with_header("content-type", "application/gzip")
        .with_body(test_package)
        .create();

    let _m2 = server.mock("GET", "/wang-q/cbp/releases/download/Binaries/zlib.linux.tar.gz")
        .with_status(200)
        .with_header("content-type", "application/gzip")
        .with_body(test_package)
        .create();

    let _m3 = server.mock("GET", "/wang-q/cbp/releases/download/Binaries/zlib.windows.tar.gz")
       .with_status(200)
       .with_header("content-type", "application/gzip")
       .with_body(test_package)
       .create();

    // 使用环境变量临时覆盖 GitHub URL
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

    // 验证安装结果
    assert!(temp_dir.path().join("include/zlib.h").exists());
    assert!(temp_dir.path().join("lib/libz.a").exists());
    assert!(temp_dir.path().join("records/zlib.files").exists());

    Ok(())
}
