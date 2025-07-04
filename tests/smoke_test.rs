use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::tempdir;
use committo::config::CONVENTION_FILE_NAME;

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
    fs::write(git_repo.join(CONVENTION_FILE_NAME), "Use conventional commits format")?;

    // Test dry-run with our built binary
    let mut cmd = Command::cargo_bin("committo")?;
    cmd.current_dir(&git_repo);
    cmd.env("OPENAI_API_KEY", "test_key_for_smoke_test");
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
    cmd.env("OPENAI_API_KEY", "test_key_for_smoke_test");
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
    let t1 = "1. For the entire project: Use the Conventional Commits format (feat, fix, docs).";
    let t2 = "2. For the frontend: When modifying UI components, the component: prefix is required.";
    let guideline = "**IMPORTANT PRIORITY RULES:**";
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

    // Create nested convention files (parent -> child priority)
    fs::write(git_repo.join(CONVENTION_FILE_NAME), "For the entire project: Use the Conventional Commits format (feat, fix, docs).")?;
    fs::write(sub_dir.join(CONVENTION_FILE_NAME), "For the frontend: When modifying UI components, the component: prefix is required.")?;

    // Create and stage a file in subdirectory
    fs::write(sub_dir.join("app.js"), "console.log('Hello');")?;
    Command::new("git")
        .current_dir(&git_repo)
        .args(["add", "."])
        .output()?;

    // Test from subdirectory
    let mut cmd = Command::cargo_bin("committo")?;
    cmd.current_dir(&sub_dir);
    cmd.env("OPENAI_API_KEY", "test_key_for_smoke_test");
    cmd.arg("generate").arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(t1))
        .stdout(predicate::str::contains(t2))
        .stdout(predicate::str::contains(guideline))
        .stdout(predicate::str::contains("app.js"));

    Ok(())
}

#[test]
fn test_numbered_priority_convention_files() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let git_repo = temp_dir.path().join("test_repo");
    let sub_dir = git_repo.join("frontend");
    let deep_dir = sub_dir.join("components");
    fs::create_dir_all(&deep_dir)?;

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

    // Create 3-level nested convention files
    fs::write(git_repo.join(CONVENTION_FILE_NAME), "Use Korean for commit messages")?;
    fs::write(sub_dir.join(CONVENTION_FILE_NAME), "Frontend: Use component prefixes")?;
    fs::write(deep_dir.join(CONVENTION_FILE_NAME), "Components: Describe UI changes in detail")?;

    // Create and stage a file in deepest directory
    fs::write(deep_dir.join("Button.js"), "export const Button = () => <button>Click</button>;")?;
    Command::new("git")
        .current_dir(&git_repo)
        .args(["add", "."])
        .output()?;

    // Test from deepest directory
    let mut cmd = Command::cargo_bin("committo")?;
    cmd.current_dir(&deep_dir);
    cmd.env("OPENAI_API_KEY", "test_key_for_smoke_test");
    cmd.arg("generate").arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("1. Use Korean for commit messages"))
        .stdout(predicate::str::contains("2. Frontend: Use component prefixes"))
        .stdout(predicate::str::contains("3. Components: Describe UI changes in detail"))
        .stdout(predicate::str::contains("**IMPORTANT PRIORITY RULES:**"))
        .stdout(predicate::str::contains("Button.js"));

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