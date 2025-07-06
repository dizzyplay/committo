use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_set_creates_new_key() {
    let temp_home = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.env("HOME", temp_home.path())
        .arg("set")
        .arg("candidate-count")
        .arg("3");
    
    cmd.assert().success();
    
    let content = fs::read_to_string(temp_home.path().join(".committo.toml")).unwrap();
    assert!(content.contains("candidate-count = 3"));
}

#[test]
fn test_set_updates_existing_key() {
    let temp_home = TempDir::new().unwrap();
    
    // Create initial config file
    fs::write(temp_home.path().join(".committo.toml"), "candidate-count = 1\napi-key = \"test\"\n").unwrap();
    
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.env("HOME", temp_home.path())
        .arg("set")
        .arg("candidate-count")
        .arg("5");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Set candidate-count = 5"));
    
    let content = fs::read_to_string(temp_home.path().join(".committo.toml")).unwrap();
    assert!(content.contains("candidate-count = 5"));
    assert!(content.contains("api-key = \"test\""));
    assert!(!content.contains("candidate-count = 1"));
}

#[test]
fn test_set_preserves_other_keys() {
    let temp_home = TempDir::new().unwrap();
    
    // Create initial config file with multiple keys
    fs::write(temp_home.path().join(".committo.toml"), "api-key = \"secret\"\nllm-model = \"gpt-4\"\n").unwrap();
    
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.env("HOME", temp_home.path())
        .arg("set")
        .arg("candidate-count")
        .arg("3");
    
    cmd.assert().success();
    
    let content = fs::read_to_string(temp_home.path().join(".committo.toml")).unwrap();
    assert!(content.contains("api-key = \"secret\""));
    assert!(content.contains("llm-model = \"gpt-4\""));
    assert!(content.contains("candidate-count = 3"));
}

#[test]
fn test_set_multiple_updates() {
    let temp_home = TempDir::new().unwrap();
    
    // Set initial value
    Command::cargo_bin("committo").unwrap()
        .env("HOME", temp_home.path())
        .arg("set").arg("candidate-count").arg("1")
        .assert().success();
    
    // Update value
    Command::cargo_bin("committo").unwrap()
        .env("HOME", temp_home.path())
        .arg("set").arg("candidate-count").arg("3")
        .assert().success();
    
    // Update again
    Command::cargo_bin("committo").unwrap()
        .env("HOME", temp_home.path())
        .arg("set").arg("candidate-count").arg("5")
        .assert().success();
    
    let content = fs::read_to_string(temp_home.path().join(".committo.toml")).unwrap();
    assert!(content.contains("candidate-count = 5"));
    assert_eq!(content.matches("candidate-count").count(), 1);
}

#[test]
fn test_show_command() {
    let temp_home = TempDir::new().unwrap();
    
    // Set some config values
    Command::cargo_bin("committo").unwrap()
        .env("HOME", temp_home.path())
        .arg("set").arg("api-key").arg("test123")
        .assert().success();
        
    Command::cargo_bin("committo").unwrap()
        .env("HOME", temp_home.path())
        .arg("set").arg("candidate-count").arg("3")
        .assert().success();
    
    // Test show command
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.env("HOME", temp_home.path())
        .arg("show");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Api Key : \"test1**\" (masked)"))
        .stdout(predicate::str::contains("Candidate Count : 3"));
}