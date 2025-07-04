use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_env_set_creates_new_key() {
    let temp_home = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.env("HOME", temp_home.path())
        .arg("env")
        .arg("set")
        .arg("CANDIDATE_COUNT=3");
    
    cmd.assert().success();
    
    let content = fs::read_to_string(temp_home.path().join(".committorc")).unwrap();
    assert!(content.contains("export CANDIDATE_COUNT=\"3\""));
}

#[test]
fn test_env_set_updates_existing_key() {
    let temp_home = TempDir::new().unwrap();
    
    // Create initial config file
    fs::write(temp_home.path().join(".committorc"), "export CANDIDATE_COUNT=\"1\"\nexport OTHER_KEY=\"value\"\n").unwrap();
    
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.env("HOME", temp_home.path())
        .arg("env")
        .arg("set")
        .arg("CANDIDATE_COUNT=5");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Updated CANDIDATE_COUNT"));
    
    let content = fs::read_to_string(temp_home.path().join(".committorc")).unwrap();
    assert!(content.contains("export CANDIDATE_COUNT=\"5\""));
    assert!(content.contains("export OTHER_KEY=\"value\""));
    assert!(!content.contains("export CANDIDATE_COUNT=\"1\""));
}

#[test]
fn test_env_set_preserves_other_keys() {
    let temp_home = TempDir::new().unwrap();
    
    // Create initial config file with multiple keys
    fs::write(temp_home.path().join(".committorc"), "export OPENAI_API_KEY=\"secret\"\nexport LLM_MODEL=\"gpt-4\"\n").unwrap();
    
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.env("HOME", temp_home.path())
        .arg("env")
        .arg("set")
        .arg("CANDIDATE_COUNT=3");
    
    cmd.assert().success();
    
    let content = fs::read_to_string(temp_home.path().join(".committorc")).unwrap();
    assert!(content.contains("export OPENAI_API_KEY=\"secret\""));
    assert!(content.contains("export LLM_MODEL=\"gpt-4\""));
    assert!(content.contains("export CANDIDATE_COUNT=\"3\""));
}

#[test]
fn test_env_set_multiple_updates() {
    let temp_home = TempDir::new().unwrap();
    
    // Set initial value
    Command::cargo_bin("committo").unwrap()
        .env("HOME", temp_home.path())
        .arg("env").arg("set").arg("CANDIDATE_COUNT=1")
        .assert().success();
    
    // Update value
    Command::cargo_bin("committo").unwrap()
        .env("HOME", temp_home.path())
        .arg("env").arg("set").arg("CANDIDATE_COUNT=3")
        .assert().success();
    
    // Update again
    Command::cargo_bin("committo").unwrap()
        .env("HOME", temp_home.path())
        .arg("env").arg("set").arg("CANDIDATE_COUNT=5")
        .assert().success();
    
    let content = fs::read_to_string(temp_home.path().join(".committorc")).unwrap();
    assert!(content.contains("export CANDIDATE_COUNT=\"5\""));
    assert_eq!(content.matches("CANDIDATE_COUNT").count(), 1);
}

#[test]
fn test_env_set_preserves_comments_and_non_export_lines() {
    let temp_home = TempDir::new().unwrap();
    
    // Create config with comments and non-export lines
    let initial_content = "# This is a comment\nexport OPENAI_API_KEY=\"secret\"\n# Another comment\necho \"Hello\"\n";
    fs::write(temp_home.path().join(".committorc"), initial_content).unwrap();
    
    Command::cargo_bin("committo").unwrap()
        .env("HOME", temp_home.path())
        .arg("env").arg("set").arg("CANDIDATE_COUNT=3")
        .assert().success();
    
    let content = fs::read_to_string(temp_home.path().join(".committorc")).unwrap();
    assert!(content.contains("# This is a comment"));
    assert!(content.contains("# Another comment"));
    assert!(content.contains("echo \"Hello\""));
    assert!(content.contains("export OPENAI_API_KEY=\"secret\""));
    assert!(content.contains("export CANDIDATE_COUNT=\"3\""));
}