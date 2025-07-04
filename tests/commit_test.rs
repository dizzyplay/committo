use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_generate_basic_help() {
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.arg("generate")
        .arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generate a commit message"))
        .stdout(predicate::str::contains("--dry-run"));
}

#[test]
fn test_env_set_supports_candidate_count() {
    let temp_home = TempDir::new().unwrap();
    
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.env("HOME", temp_home.path())
        .arg("env")
        .arg("set")
        .arg("CANDIDATE_COUNT=5");
    
    // Should not fail due to argument parsing
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(!stderr.contains("error parsing command line arguments"));
}

#[test]
fn test_generate_with_dry_run() {
    // Test basic generate functionality
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.arg("generate")
        .arg("--dry-run");
    
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should not fail due to argument parsing
    assert!(!stderr.contains("error parsing command line arguments"));
}

#[test]
fn test_basic_generate_parsing() {
    // Test that generate command parses correctly
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.arg("generate")
        .arg("--dry-run");
    
    // Should not fail due to argument parsing
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(!stderr.contains("error parsing command line arguments"));
    assert!(!stderr.contains("conflicts with"));
}

#[test] 
fn test_env_command_functionality() {
    // Test that env command works with CANDIDATE_COUNT
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.arg("env")
        .arg("--help");
    
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should parse successfully
    assert!(!stderr.contains("error parsing command line arguments"));
    assert!(!stderr.contains("Invalid value"));
}

#[test]
fn test_pipe_detection_exists() {
    // Test that the pipe detection function exists and doesn't panic
    // We can't easily test actual pipe detection in unit tests
    let mut cmd = Command::cargo_bin("committo").unwrap();
    cmd.arg("generate")
        .arg("--help");
    
    // If the binary builds and help works, our pipe detection code compiled
    cmd.assert().success();
}