use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn smoke_test_full_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let git_repo = temp_dir.path().join("test_repo");
    fs::create_dir(&git_repo)?;

    // Initialize git repo
    Command::new("git")
        .current_dir(&git_repo)
        .args(["init"])
        .output()?;

    // Configure git (required for commits)
    Command::new("git")
        .current_dir(&git_repo)
        .args(["config", "user.name", "Test User"])
        .output()?;

    Command::new("git")
        .current_dir(&git_repo)
        .args(["config", "user.email", "test@example.com"])
        .output()?;

    // Create a test file and make changes
    fs::write(git_repo.join("test.txt"), "Hello, World!")?;
    
    // Stage the file
    Command::new("git")
        .current_dir(&git_repo)
        .args(["add", "test.txt"])
        .output()?;

    // Create convention file
    fs::write(git_repo.join(".committoconvention"), "Use conventional commits format")?;

    // Test dry-run with our built binary
    let mut cmd = Command::cargo_bin("committo")?;
    cmd.current_dir(&git_repo);
    cmd.arg("generate").arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--- Dry Run ---"))
        .stdout(predicate::str::contains("--- Prompt ---"))
        .stdout(predicate::str::contains("Use conventional commits format"))
        .stdout(predicate::str::contains("--- Git Diff ---"))
        .stdout(predicate::str::contains("test.txt"));

    Ok(())
}

#[test]
fn smoke_test_multiple_files() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let git_repo = temp_dir.path().join("test_repo");
    fs::create_dir(&git_repo)?;

    // Initialize git repo
    Command::new("git")
        .current_dir(&git_repo)
        .args(["init"])
        .output()?;

    Command::new("git")
        .current_dir(&git_repo)
        .args(["config", "user.name", "Test User"])
        .output()?;

    Command::new("git")
        .current_dir(&git_repo)
        .args(["config", "user.email", "test@example.com"])
        .output()?;

    // Create multiple files
    fs::create_dir_all(git_repo.join("src"))?;
    fs::write(git_repo.join("src/main.rs"), "fn main() { println!(\"Hello\"); }")?;
    fs::write(git_repo.join("Cargo.toml"), "[package]\nname = \"test\"\nversion = \"0.1.0\"")?;
    fs::write(git_repo.join("README.md"), "# Test Project")?;

    // Stage files
    Command::new("git")
        .current_dir(&git_repo)
        .args(["add", "."])
        .output()?;

    // Test dry-run
    let mut cmd = Command::cargo_bin("committo")?;
    cmd.current_dir(&git_repo);
    cmd.arg("generate").arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--- Dry Run ---"))
        .stdout(predicate::str::contains("main.rs"))
        .stdout(predicate::str::contains("Cargo.toml"));

    Ok(())
}

#[test]
fn smoke_test_nested_convention_files() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let git_repo = temp_dir.path().join("test_repo");
    let sub_dir = git_repo.join("frontend");
    fs::create_dir_all(&sub_dir)?;

    // Initialize git repo
    Command::new("git")
        .current_dir(&git_repo)
        .args(["init"])
        .output()?;

    Command::new("git")
        .current_dir(&git_repo)
        .args(["config", "user.name", "Test User"])
        .output()?;

    Command::new("git")
        .current_dir(&git_repo)
        .args(["config", "user.email", "test@example.com"])
        .output()?;

    // Create nested convention files
    fs::write(git_repo.join(".committoconvention"), "Root: Use semantic versioning")?;
    fs::write(sub_dir.join(".committoconvention"), "Frontend: Use UI/UX prefixes")?;

    // Create and stage a file in subdirectory
    fs::write(sub_dir.join("app.js"), "console.log('Hello');")?;
    Command::new("git")
        .current_dir(&git_repo)
        .args(["add", "."])
        .output()?;

    // Test from subdirectory
    let mut cmd = Command::cargo_bin("committo")?;
    cmd.current_dir(&sub_dir);
    cmd.arg("generate").arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Root: Use semantic versioning"))
        .stdout(predicate::str::contains("Frontend: Use UI/UX prefixes"))
        .stdout(predicate::str::contains("app.js"));

    Ok(())
}

/// Quick helper function to run smoke tests easily
pub fn run_smoke_test_helper() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running smoke tests...");
    
    // Build the project first
    Command::new("cargo")
        .args(["build"])
        .output()?;
    
    // Run specific smoke tests
    Command::new("cargo")
        .args(["test", "--test", "smoke_test"])
        .output()?;
    
    println!("Smoke tests completed!");
    Ok(())
}