use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_set_command() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join(".comittoorc");

    // Mock the HOME environment variable to control where the .comittoorc file is created.
    let mut cmd = Command::cargo_bin("committo")?;
    cmd.env("HOME", temp_dir.path());
    cmd.arg("env").arg("set").arg("OPENAI_API=test_key");
    
    cmd.assert().success();

    let content = fs::read_to_string(config_path)?;
    assert!(content.contains("export OPENAI_API=\"test_key\""));

    Ok(())
}

#[test]
fn test_generate_dry_run_with_convention_files() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let project_root = temp_dir.path();
    let sub_dir = project_root.join("subdir");
    fs::create_dir(&sub_dir)?;

    // Create convention files
    fs::write(project_root.join(".comittoconvention"), "Root convention")?;
    fs::write(sub_dir.join(".comittoconvention"), "Subdir convention")?;

    // Mock git diff by creating a dummy script
    let script_path = temp_dir.path().join("git");
    fs::write(&script_path, "#!/bin/sh\necho \"diff --git a/file.txt b/file.txt\"\nexit 0")?;
    fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755))?;

    let mut cmd = Command::cargo_bin("committo")?;
    cmd.current_dir(&sub_dir);
    cmd.env("PATH", format!("{}:{}", temp_dir.path().to_str().unwrap(), std::env::var("PATH").unwrap())); // Prepend the temp dir to PATH
    cmd.arg("generate").arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--- Dry Run ---"))
        .stdout(predicate::str::contains("API Key Source: .comittoorc file")) // Assuming it's set in the test env
        .stdout(predicate::str::contains("--- Prompt ---"))
        .stdout(predicate::str::contains("Root convention\n\nSubdir convention"))
        .stdout(predicate::str::contains("--- Git Diff ---"))
        .stdout(predicate::str::contains("diff --git a/file.txt b/file.txt"));

    Ok(())
}

#[test]
fn test_show_command_with_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join(".comittoorc");
    fs::write(&config_path, "export TEST_KEY=\"test_value\"")?;

    let mut cmd = Command::cargo_bin("committo")?;
    cmd.env("HOME", temp_dir.path());
    cmd.arg("env").arg("show");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("TEST_KEY: test_value"));

    Ok(())
}

#[test]
fn test_show_command_without_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;

    let mut cmd = Command::cargo_bin("committo")?;
    cmd.env("HOME", temp_dir.path());
    cmd.arg("env").arg("show");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!("No .comittoorc file found at {}.", temp_dir.path().join(".comittoorc").display())));

    Ok(())
}