use committo::providers::{OpenAiProvider, ProviderFactory};
use committo::api::{LlmProvider, generate_commit_message_with_provider};
use committo::config::{DEFAULT_OPENAI_MODEL, GPT4_MODEL, Config};

#[path = "common/mock.rs"]
mod mock;
use mock::MockProvider;

#[cfg(test)]
mod provider_tests {
    use super::*;

    #[test]
    fn test_openai_provider_config() {
        let app_config = Config::default();
        let provider = OpenAiProvider::new(app_config);
        let config = provider.get_config();
        
        assert_eq!(config.model, DEFAULT_OPENAI_MODEL);
        assert_eq!(config.endpoint, "https://api.openai.com/v1/chat/completions");
        assert_eq!(provider.get_provider_name(), "OpenAI");
    }

    #[test]
    fn test_openai_provider_with_custom_model() {
        let app_config = Config::default();
        let provider = OpenAiProvider::with_model(app_config, "gpt-4");
        let config = provider.get_config();
        
        assert_eq!(config.model, GPT4_MODEL);
        assert_eq!(provider.get_provider_name(), "OpenAI");
    }

    #[test]
    fn test_mock_provider_config() {
        let provider = MockProvider::new();
        let config = provider.get_config();
        
        assert_eq!(config.model, "mock-model");
        assert_eq!(provider.get_provider_name(), "mock");
        assert_eq!(provider.get_candidate_count(), 1);
    }

    #[test]
    fn test_api_key_masking() {
        let config = Config {
            api_key: Some("test_key".to_string()),
            candidate_count: Some(1),
            llm_provider: Some("mock".to_string()),
            llm_model: Some("mock-model".to_string()),
            run_edit: Some(true),
        };
        
        // Test various key lengths
        assert_eq!(config.mask_api_key("sk-1234567890"), "sk-12********");
        assert_eq!(config.mask_api_key("abcd"), "****");
        assert_eq!(config.mask_api_key("12345"), "12345");
        assert_eq!(config.mask_api_key("123456789012345"), "12345**********");
    }

    #[test]
    fn test_config_dry_run_output() {
        let config = Config {
            api_key: Some("test_key".to_string()),
            candidate_count: Some(5),
            llm_provider: Some("openai".to_string()),
            llm_model: Some("gpt-4".to_string()),
            run_edit: Some(true),
        };
        
        let output = config.show_masking_config();
        assert!(output.contains("--- Configuration ---"));
        assert!(output.contains("Candidate Count : 5"));
        assert!(output.contains("LLM Provider : \"openai\""));
        assert!(output.contains("LLM Model : \"gpt-4\""));
        assert!(output.contains("(masked)"));
    }

    #[tokio::test]
    async fn test_mock_provider_success() {
        let provider = MockProvider::with_response("feat: add new feature");
        
        let result = provider.generate_commit_message_impl("Test prompt", "Test diff").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "feat: add new feature");
    }

    #[tokio::test]
    async fn test_mock_provider_failure() {
        let provider = MockProvider::with_failure();
        
        let result = provider.generate_commit_message_impl("Test prompt", "Test diff").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Mock API error"));
    }

    #[tokio::test]
    async fn test_dry_run_mode() {
        let provider = MockProvider::with_response("test response");
        
        let result = provider.generate_commit_message("diff content", true).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Dry run complete.");
    }

    #[tokio::test]
    async fn test_generate_commit_message_with_provider() {
        let provider = MockProvider::with_response("feat: implement new feature");
        
        let result = generate_commit_message_with_provider(
            &provider,
            "diff content",
            false
        ).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "feat: implement new feature");
    }

    #[test]
    fn test_provider_factory_default() {
        // Test default provider creation
        let config = Config::default();
        let provider = ProviderFactory::create_provider(config);
        assert_eq!(provider.get_provider_name(), "OpenAI");
    }

    #[test]
    fn test_provider_factory_specific_methods() {
        let config = Config::default();
        let provider = ProviderFactory::create_openai(config.clone());
        assert_eq!(provider.get_provider_name(), "OpenAI");
        assert_eq!(provider.get_config().model, DEFAULT_OPENAI_MODEL);
        
        let provider = ProviderFactory::create_openai_with_model(config, "gpt-4-turbo");
        assert_eq!(provider.get_provider_name(), "OpenAI");
        assert_eq!(provider.get_config().model, "gpt-4-turbo");
    }
}