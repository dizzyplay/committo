use committo::api::generate_commit_message_with_provider;

#[path = "common/mock.rs"]
mod mock;
use mock::{MockProvider, MockConfig};

#[tokio::test]
async fn test_mock_provider_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Setup mock provider
    let provider = MockProvider::with_response("feat: add mock integration test");

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
    Ok(())
}

#[tokio::test]
async fn test_provider_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Test with failing provider
    let provider = MockProvider::with_failure();

    let result = generate_commit_message_with_provider(
        &provider,
        "diff content",
        false
    ).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Mock API error"));
    Ok(())
}

#[tokio::test]
async fn test_provider_without_api_key() -> Result<(), Box<dyn std::error::Error>> {
    // Test without API key
    let config = MockConfig {
        api_key: None,
        candidate_count: Some(1),
        dev_mode: Some(false),
    };
    let provider = MockProvider::with_config(config);

    let result = generate_commit_message_with_provider(
        &provider,
        "diff content",
        false
    ).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("API key not found"));

    Ok(())
}