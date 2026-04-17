use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn command_dot_help() -> anyhow::Result<()> {
    Command::cargo_bin("cbp")?
        .arg("dot")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage dotfiles with templates"))
        .stdout(predicate::str::contains("private_"))
        .stdout(predicate::str::contains("executable_"))
        .stdout(predicate::str::contains("dot_"));

    Ok(())
}

#[test]
fn command_dot_create_template() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let dotfiles_dir = temp_dir.path().join("dotfiles");
    let source_file = temp_dir.path().join(".bashrc");

    // Create a source config file
    std::fs::write(&source_file, "alias ls='ls --color=auto'\n")?;

    // Create template from source
    Command::cargo_bin("cbp")?
        .arg("dot")
        .arg(&source_file)
        .arg("--dir")
        .arg(&dotfiles_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created:"));

    // Verify template was created
    let template_file = dotfiles_dir.join("dot_bashrc.tmpl");
    assert!(template_file.exists());

    // Verify content
    let content = std::fs::read_to_string(&template_file)?;
    assert!(content.contains("alias ls"));

    Ok(())
}

#[test]
fn command_dot_preview_template() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let dotfiles_dir = temp_dir.path().join("dotfiles");
    std::fs::create_dir_all(&dotfiles_dir)?;

    // Create a template file
    let template_file = dotfiles_dir.join("dot_bashrc.tmpl");
    std::fs::write(
        &template_file,
        "# Test bashrc\nexport PATH=$HOME/bin:$PATH\n",
    )?;

    // Preview template (without --apply)
    Command::cargo_bin("cbp")?
        .arg("dot")
        .arg(&template_file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Preview:"))
        .stdout(predicate::str::contains("Use -a or --apply"));

    Ok(())
}

#[test]
fn command_dot_apply_template() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let dotfiles_dir = temp_dir.path().join("dotfiles");
    std::fs::create_dir_all(&dotfiles_dir)?;

    // Create a template file
    let template_file = dotfiles_dir.join("dot_testrc.tmpl");
    std::fs::write(&template_file, "# Test config\nvalue=123\n")?;

    // Apply template with --apply - just verify it succeeds
    // Note: dirs::home_dir() uses system APIs, not HOME env var on Windows
    // So we just verify the command succeeds and outputs correctly
    let output = Command::cargo_bin("cbp")?
        .arg("dot")
        .arg(&template_file)
        .arg("--apply")
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("stdout: {}", stdout);
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // Verify command succeeded
    assert!(output.status.success(), "Command failed: {}", stdout);
    assert!(
        stdout.contains("Applied:"),
        "Expected 'Applied:' in output: {}",
        stdout
    );

    Ok(())
}

#[test]
fn command_dot_template_with_variables() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let dotfiles_dir = temp_dir.path().join("dotfiles");
    let home_dir = temp_dir.path().join("home");
    std::fs::create_dir_all(&dotfiles_dir)?;
    std::fs::create_dir_all(&home_dir)?;

    // Create a template with variables
    let template_file = dotfiles_dir.join("dot_test.tmpl");
    std::fs::write(
        &template_file,
        "# OS: {% if os == \"windows\" %}Windows{% else %}Other{% endif %}\n",
    )?;

    // Preview template
    Command::cargo_bin("cbp")?
        .arg("dot")
        .arg(&template_file)
        .env("HOME", &home_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("OS:"));

    Ok(())
}

#[test]
fn command_dot_apply_xdg_config() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let dotfiles_dir = temp_dir.path().join("dotfiles");
    let home_dir = temp_dir.path().join("home");
    std::fs::create_dir_all(&dotfiles_dir)?;
    std::fs::create_dir_all(&home_dir)?;

    // Create xdg_config directory structure
    let config_dir = dotfiles_dir.join("xdg_config");
    std::fs::create_dir_all(&config_dir)?;
    let template_file = config_dir.join("myapp/config.toml");
    std::fs::create_dir_all(template_file.parent().unwrap())?;
    std::fs::write(&template_file, "[settings]\nkey = \"value\"\n")?;

    // Apply template
    Command::cargo_bin("cbp")?
        .arg("dot")
        .arg(&template_file)
        .arg("--apply")
        .env("HOME", &home_dir)
        .env("APPDATA", home_dir.join("AppData/Roaming"))
        .assert()
        .success();

    Ok(())
}

#[test]
fn command_dot_export_and_apply_archive() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let source_dir = temp_dir.path().join("source");
    let archive_path = temp_dir.path().join("test.tar.gz");
    let home_dir = temp_dir.path().join("home");
    std::fs::create_dir_all(&source_dir)?;
    std::fs::create_dir_all(&home_dir)?;

    // Create source files
    std::fs::write(source_dir.join("file1.txt"), "content1")?;
    std::fs::write(source_dir.join("file2.txt"), "content2")?;

    // Export to archive
    Command::cargo_bin("cbp")?
        .arg("dot")
        .arg(&source_dir)
        .arg("--tar")
        .arg(&archive_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created:"));

    // Verify archive was created
    assert!(archive_path.exists());

    // Preview archive application
    Command::cargo_bin("cbp")?
        .arg("dot")
        .arg(&archive_path)
        .env("HOME", &home_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Preview:"));

    Ok(())
}

#[test]
fn command_dot_conflicting_options() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let source_file = temp_dir.path().join("test.txt");
    std::fs::write(&source_file, "test")?;

    // Test --apply and --dir conflict
    Command::cargo_bin("cbp")?
        .arg("dot")
        .arg(&source_file)
        .arg("--apply")
        .arg("--dir")
        .arg(temp_dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Cannot use --apply and --dir"));

    Ok(())
}

#[test]
fn command_dot_verbose_output() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let dotfiles_dir = temp_dir.path().join("dotfiles");
    let source_file = temp_dir.path().join(".bashrc");
    std::fs::create_dir_all(&dotfiles_dir)?;
    std::fs::write(&source_file, "# bashrc\n")?;

    // Create template with verbose
    Command::cargo_bin("cbp")?
        .arg("dot")
        .arg(&source_file)
        .arg("--dir")
        .arg(&dotfiles_dir)
        .arg("--verbose")
        .assert()
        .success()
        .stdout(predicate::str::contains("Source:"))
        .stdout(predicate::str::contains("Template:"));

    Ok(())
}

#[test]
fn command_dot_nonexistent_source() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let nonexistent = temp_dir.path().join("nonexistent.txt");

    // Try to create template from non-existent file
    Command::cargo_bin("cbp")?
        .arg("dot")
        .arg(&nonexistent)
        .arg("--dir")
        .arg(temp_dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));

    Ok(())
}

/// Test complete workflow: create template -> preview -> apply -> verify content
#[test]
fn command_dot_full_workflow() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let source_dir = temp_dir.path().join("source");
    let templates_dir = temp_dir.path().join("templates");
    std::fs::create_dir_all(&source_dir)?;

    // Step 1: Create a source config file with template variables
    let source_file = source_dir.join("config.txt");
    std::fs::write(
        &source_file,
        "# Config for {{ os }}\nuser={{ user }}\n",
    )?;

    // Step 2: Create template from source
    Command::cargo_bin("cbp")?
        .arg("dot")
        .arg(&source_file)
        .arg("--dir")
        .arg(&templates_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created:"));

    // Verify template was created with .tmpl extension
    let template_file = templates_dir.join("config.txt.tmpl");
    assert!(template_file.exists(), "Template file should be created");

    // Verify template content preserved the template syntax
    let template_content = std::fs::read_to_string(&template_file)?;
    assert!(
        template_content.contains("{{ os }}"),
        "Template should contain {{ os }} variable"
    );
    assert!(
        template_content.contains("{{ user }}"),
        "Template should contain {{ user }} variable"
    );

    // Step 3: Preview the template - should show rendered content
    let output = Command::cargo_bin("cbp")?
        .arg("dot")
        .arg(&template_file)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Preview output:\n{}", stdout);

    assert!(output.status.success(), "Preview should succeed");
    assert!(
        stdout.contains("Preview:"),
        "Should show preview message"
    );
    assert!(
        stdout.contains("Rendered content"),
        "Should show rendered content section"
    );
    // The rendered content should have replaced the variables
    assert!(
        !stdout.contains("{{ os }}"),
        "Rendered content should not contain raw {{ os }} variable"
    );

    Ok(())
}

/// Test template with conditional rendering based on OS
#[test]
fn command_dot_template_conditional_os() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let templates_dir = temp_dir.path().join("templates");
    std::fs::create_dir_all(&templates_dir)?;

    // Create template with OS-specific content
    let template_file = templates_dir.join("dot_shellrc.tmpl");
    std::fs::write(
        &template_file,
        r#"# Shell configuration
{% if os == "windows" %}
alias ls='ls -F'
{% else %}
alias ls='ls --color=auto'
{% endif %}
# Generated for {{ user }} on {{ hostname }}
"#,
    )?;

    // Preview and capture output
    let output = Command::cargo_bin("cbp")?
        .arg("dot")
        .arg(&template_file)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Conditional template output:\n{}", stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Shell configuration"));

    // Verify conditional rendering - should show only one alias based on OS
    let is_windows = stdout.contains("alias ls='ls -F'");
    let is_unix = stdout.contains("alias ls='ls --color=auto'");
    assert!(
        is_windows || is_unix,
        "Should render one of the OS-specific aliases"
    );
    assert!(
        !(is_windows && is_unix),
        "Should not render both aliases"
    );

    // Verify user and hostname are rendered (not empty)
    assert!(
        stdout.contains("Generated for"),
        "Should contain 'Generated for' text"
    );
    // The line should not contain raw template variables
    assert!(
        !stdout.contains("{{ user }}"),
        "Should not contain raw {{ user }} variable"
    );
    assert!(
        !stdout.contains("{{ hostname }}"),
        "Should not contain raw {{ hostname }} variable"
    );

    Ok(())
}

/// Test executable_ prefix sets correct permissions
#[test]
fn command_dot_executable_prefix() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let templates_dir = temp_dir.path().join("templates");
    std::fs::create_dir_all(&templates_dir)?;

    // Create executable template
    let template_file = templates_dir.join("executable_dot_script.sh.tmpl");
    std::fs::write(&template_file, "#!/bin/bash\necho 'Hello {{ user }}'\n")?;

    // Preview and verify permissions are shown as Executable
    let output = Command::cargo_bin("cbp")?
        .arg("dot")
        .arg(&template_file)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Executable template output:\n{}", stdout);

    assert!(output.status.success());
    assert!(
        stdout.contains("Executable") || stdout.contains("0755"),
        "Should show Executable permissions (0755)"
    );

    Ok(())
}

/// Test private_ prefix sets correct permissions
#[test]
fn command_dot_private_prefix() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let templates_dir = temp_dir.path().join("templates");
    std::fs::create_dir_all(&templates_dir)?;

    // Create private template
    let template_file = templates_dir.join("private_dot_ssh_config.tmpl");
    std::fs::write(&template_file, "Host example\n  User {{ user }}\n")?;

    // Preview and verify permissions are shown as Private
    let output = Command::cargo_bin("cbp")?
        .arg("dot")
        .arg(&template_file)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Private template output:\n{}", stdout);

    assert!(output.status.success());
    assert!(
        stdout.contains("Private") || stdout.contains("0600"),
        "Should show Private permissions (0600)"
    );

    Ok(())
}

/// Test xdg_config/ prefix maps to correct target path
#[test]
fn command_dot_xdg_config_prefix() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let templates_dir = temp_dir.path().join("templates");
    std::fs::create_dir_all(&templates_dir)?;

    // Create xdg_config template
    let config_dir = templates_dir.join("xdg_config");
    let myapp_dir = config_dir.join("myapp");
    std::fs::create_dir_all(&myapp_dir)?;
    let template_file = myapp_dir.join("settings.toml");
    std::fs::write(&template_file, "[app]\nname = \"{{ user }}\"\n")?;

    // Preview and verify target path
    let output = Command::cargo_bin("cbp")?
        .arg("dot")
        .arg(&template_file)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("xdg_config template output:\n{}", stdout);

    assert!(output.status.success());
    // Target should contain myapp/settings.toml (without xdg_config/ prefix)
    // Use path separator that works on both platforms
    let path_matched = stdout.contains("myapp/settings.toml")
        || stdout.contains("myapp\\settings.toml");
    assert!(
        path_matched,
        "Target path should contain myapp/settings.toml, got: {}",
        stdout
    );

    Ok(())
}
