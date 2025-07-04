use committo::api::generate_commit_message_with_provider;
use self::providers::MockProvider;

mod providers;
use std::env;
use serial_test::serial;
use tempfile::TempDir;

const MOCK_API_KEY_ENV: &str = "MOCK_API_KEY";

#[tokio::test(flavor = "current_thread")]
#[serial]
async fn test_mock_provider_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Setup mock provider
    let provider = MockProvider::with_response("feat: add mock integration test");
    unsafe { env::set_var(MOCK_API_KEY_ENV, "test_key_12345"); }
    
    // Test dry run
    let dry_result = generate_commit_message_with_provider(
        &provider,
        "diff --git a/test.txt b/test.txt\n+new line",
        true,
        1
    ).await?;
    assert_eq!(dry_result, "Dry run complete.");
    
    // Test actual generation
    let result = generate_commit_message_with_provider(
        &provider,
        "diff --git a/test.txt b/test.txt\n+new line",
        false,
        1
    ).await?;
    assert_eq!(result, "feat: add mock integration test");
    
    unsafe { env::remove_var(MOCK_API_KEY_ENV); }
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
#[serial]
async fn test_provider_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Test with failing provider
    let provider = MockProvider::with_failure();
    unsafe { env::set_var(MOCK_API_KEY_ENV, "test_key"); }
    
    let result = generate_commit_message_with_provider(
        &provider,
        "diff content",
        false,
        1
    ).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Mock API error"));
    
    unsafe { env::remove_var(MOCK_API_KEY_ENV); }
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
#[serial]
async fn test_provider_without_api_key() -> Result<(), Box<dyn std::error::Error>> {
    // Test without API key - use temporary home directory with no config file
    let temp_home = TempDir::new().unwrap();
    let _old_home = env::var("HOME");
    
    // Set HOME to temp directory (no config file)
    unsafe { env::set_var("HOME", temp_home.path()); }
    
    // Remove environment variable
    unsafe { env::remove_var(MOCK_API_KEY_ENV); }
    
    let provider = MockProvider::new();
    let result = generate_commit_message_with_provider(
        &provider,
        "diff content",
        false,
        1
    ).await;
    
    // Restore HOME
    if let Ok(old_home) = _old_home {
        unsafe { env::set_var("HOME", old_home); }
    }
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("API key not found"));
    
    Ok(())
}