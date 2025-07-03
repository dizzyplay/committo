use committo::api::generate_commit_message_with_provider;
use self::providers::MockProvider;

mod providers;
use std::env;
use serial_test::serial;

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
        true
    ).await?;
    assert_eq!(dry_result, "Dry run complete.");
    
    // Test actual generation
    let result = generate_commit_message_with_provider(
        &provider,
        "diff --git a/test.txt b/test.txt\n+new line",
        false
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
        false
    ).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Mock API error"));
    
    unsafe { env::remove_var(MOCK_API_KEY_ENV); }
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
#[serial]
async fn test_provider_without_api_key() -> Result<(), Box<dyn std::error::Error>> {
    // Test without API key
    let provider = MockProvider::new();
    unsafe { env::remove_var(MOCK_API_KEY_ENV); }
    
    let result = generate_commit_message_with_provider(
        &provider,
        "diff content",
        false
    ).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains(&format!("{} environment variable not set", MOCK_API_KEY_ENV)));
    
    Ok(())
}