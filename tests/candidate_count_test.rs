use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_set_candidate_count() {
    let temp_home = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.env("HOME", temp_home.path())
        .arg("set")
        .arg("candidate-count")
        .arg("3");
    
    // Should not fail due to argument parsing
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(!stderr.contains("error parsing command line arguments"));
}

#[test]
fn test_help_shows_set_usage() {
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.arg("set")
        .arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("config"));
}

#[test]
fn test_candidate_count_with_config() {
    // Test that candidate count can be set via config and used with generate
    let temp_home = TempDir::new().unwrap();
    
    // First set candidate count in config
    let mut set_cmd = Command::cargo_bin("committo").unwrap();
    set_cmd.env("HOME", temp_home.path())
        .arg("set")
        .arg("candidate-count")
        .arg("3");
    set_cmd.assert().success();
    
    // Then test generate with dry-run
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.env("HOME", temp_home.path())
        .arg("generate")
        .arg("--dry-run");
    
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should not have CLI parsing errors
    assert!(!stderr.contains("error parsing command line arguments") && 
            !stderr.contains("Invalid value") &&
            !stderr.contains("unrecognized subcommand"));
}

#[test]
fn test_candidate_count_validation() {
    let temp_home = TempDir::new().unwrap();
    
    // Test setting invalid candidate count
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.env("HOME", temp_home.path())
        .arg("set")
        .arg("candidate-count")
        .arg("invalid");
    
    // Should fail with parsing error
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("must be a number"));
}

#[test]
fn test_basic_generate_parsing() {
    // Test that generate command parses correctly without any config
    let temp_home = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.env("HOME", temp_home.path())
        .arg("generate")
        .arg("--dry-run");
    
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should not have CLI parsing errors
    assert!(!stderr.contains("error parsing command line arguments") && 
            !stderr.contains("unrecognized subcommand"));
}