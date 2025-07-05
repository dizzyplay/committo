use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use tempfile::tempdir;
use committo::config::{CONFIG_FILE_NAME, CONVENTION_FILE_NAME};

#[test]
fn test_set_command() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join(CONFIG_FILE_NAME);

    // Mock the HOME environment variable to control where the config file is created.
    let mut cmd = Command::cargo_bin("committo")?;
    cmd.env("HOME", temp_dir.path());
    cmd.arg("set").arg("api-key").arg("test_key");
    
    cmd.assert().success();

    let content = fs::read_to_string(config_path)?;
    assert!(content.contains("api-key = \"test_key\""));

    Ok(())
}

#[test]
fn test_generate_dry_run_with_convention_files() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let project_root = temp_dir.path();
    let sub_dir = project_root.join("subdir");
    fs::create_dir(&sub_dir)?;

    // Create convention files
    fs::write(project_root.join(CONVENTION_FILE_NAME), "Root convention")?;
    fs::write(sub_dir.join(CONVENTION_FILE_NAME), "Subdir convention")?;

    // Mock git diff by creating a dummy script
    let script_path = temp_dir.path().join("git");
    fs::write(&script_path, "#!/bin/sh\necho \"diff --git a/file.txt b/file.txt\"\nexit 0")?;
    fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755))?;

    // Create config file with API key
    fs::write(project_root.join("committo.toml"), "api-key = \"test_key_for_dry_run\"")?;

    let mut cmd = Command::cargo_bin("committo")?;
    cmd.current_dir(&sub_dir);
    cmd.env("PATH", format!("{}:{}", temp_dir.path().to_str().unwrap(), std::env::var("PATH").unwrap())); // Prepend the temp dir to PATH
    cmd.env("HOME", project_root); // Set HOME to project root so it finds our config
    cmd.arg("generate").arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--- Dry Run ---"))
        .stdout(predicate::str::contains("--- Configuration ---"))
        .stdout(predicate::str::contains("Api Key :"))
        .stdout(predicate::str::contains("(masked)"))
        .stdout(predicate::str::contains("--- Prompt ---"))
        .stdout(predicate::str::contains("1. Root convention"))
        .stdout(predicate::str::contains("2. Subdir convention"))
        .stdout(predicate::str::contains("**IMPORTANT PRIORITY RULES:**"))
        .stdout(predicate::str::contains("--- Git Diff ---"))
        .stdout(predicate::str::contains("diff --git a/file.txt b/file.txt"));

    Ok(())
}

#[test]
fn test_show_command_with_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join(CONFIG_FILE_NAME);
    fs::write(&config_path, "api-key = \"test_value\"")?;

    let mut cmd = Command::cargo_bin("committo")?;
    cmd.env("HOME", temp_dir.path());
    cmd.arg("show");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Api Key"));

    Ok(())
}

#[test]
fn test_show_command_without_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;

    let mut cmd = Command::cargo_bin("committo")?;
    cmd.env("HOME", temp_dir.path());
    cmd.arg("show");

    cmd.assert()
        .stderr(predicate::str::contains("Error: IO error: not a terminal"));

    Ok(())
}