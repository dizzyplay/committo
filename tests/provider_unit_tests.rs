use committo::providers::{OpenAiProvider, ProviderFactory};
use committo::api::{LlmProvider, generate_commit_message_with_provider};
use committo::config::{CONFIG_FILE_NAME, OPENAI_API_KEY_ENV, DEFAULT_OPENAI_MODEL, GPT4_MODEL, LLM_PROVIDER_ENV, PROVIDER_OPENAI, PROVIDER_OPENAI_GPT4};
use self::providers::MockProvider;

mod providers;
use std::env;
use serial_test::serial;

const MOCK_API_KEY_ENV: &str = "MOCK_API_KEY";

#[cfg(test)]
mod provider_tests {
    use super::*;

    #[test]
    fn test_openai_provider_config() {
        let provider = OpenAiProvider::new();
        let config = provider.get_config();
        
        assert_eq!(config.api_key_env_var, OPENAI_API_KEY_ENV);
        assert_eq!(config.model, DEFAULT_OPENAI_MODEL);
        assert_eq!(config.endpoint, "https://api.openai.com/v1/chat/completions");
        assert_eq!(provider.get_provider_name(), "OpenAI");
    }

    #[test]
    fn test_openai_provider_with_custom_model() {
        let provider = OpenAiProvider::with_model("gpt-4");
        let config = provider.get_config();
        
        assert_eq!(config.model, GPT4_MODEL);
        assert_eq!(config.api_key_env_var, OPENAI_API_KEY_ENV);
        assert_eq!(provider.get_provider_name(), "OpenAI");
    }

    #[test]
    fn test_mock_provider_config() {
        let provider = MockProvider::new();
        let config = provider.get_config();
        
        assert_eq!(config.api_key_env_var, MOCK_API_KEY_ENV);
        assert_eq!(config.model, "mock-model");
        assert_eq!(provider.get_provider_name(), "Mock");
    }

    #[test]
    fn test_api_key_masking() {
        let provider = MockProvider::new();
        
        // Test various key lengths
        assert_eq!(provider.mask_api_key("sk-1234567890"), "sk-12********");
        assert_eq!(provider.mask_api_key("abcd"), "****");
        assert_eq!(provider.mask_api_key("12345"), "12345");
        assert_eq!(provider.mask_api_key("123456789012345"), "12345**********");
    }

    #[test]
    #[serial]
    fn test_api_key_source_detection() {
        let provider = MockProvider::new();
        
        // Test when env var is not set
        unsafe { env::remove_var(MOCK_API_KEY_ENV); }
        assert_eq!(provider.get_api_key_source(), format!("{} file", CONFIG_FILE_NAME));
        
        // Test when env var is set
        unsafe { env::set_var(MOCK_API_KEY_ENV, "test_key"); }
        assert_eq!(provider.get_api_key_source(), "Environment variable");
        
        // Cleanup
        unsafe { env::remove_var(MOCK_API_KEY_ENV); }
    }

    #[tokio::test(flavor = "current_thread")]
    #[serial]
    async fn test_mock_provider_success() {
        let provider = MockProvider::with_response("feat: add new feature");
        
        unsafe { env::set_var(MOCK_API_KEY_ENV, "test_key"); }
        let result = provider.generate_commit_message_impl("Test prompt", "Test diff").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "feat: add new feature");
        unsafe { env::remove_var(MOCK_API_KEY_ENV); }
    }

    #[tokio::test(flavor = "current_thread")]
    #[serial]
    async fn test_mock_provider_failure() {
        let provider = MockProvider::with_failure();
        
        unsafe { env::set_var(MOCK_API_KEY_ENV, "test_key"); }
        let result = provider.generate_commit_message_impl("Test prompt", "Test diff").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Mock API error"));
        unsafe { env::remove_var(MOCK_API_KEY_ENV); }
    }

    #[tokio::test(flavor = "current_thread")]
    #[serial]
    async fn test_dry_run_mode() {
        let provider = MockProvider::with_response("test response");
        
        unsafe { env::set_var(MOCK_API_KEY_ENV, "sk-test123456789"); }
        let result = provider.generate_commit_message("diff content", true).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Dry run complete.");
        unsafe { env::remove_var(MOCK_API_KEY_ENV); }
    }

    #[tokio::test(flavor = "current_thread")]
    #[serial]
    async fn test_generate_commit_message_with_provider() {
        let provider = MockProvider::with_response("feat: implement new feature");
        
        unsafe { env::set_var(MOCK_API_KEY_ENV, "test_key"); }
        let result = generate_commit_message_with_provider(
            &provider,
            "diff content",
            false
        ).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "feat: implement new feature");
        unsafe { env::remove_var(MOCK_API_KEY_ENV); }
    }

    #[test]
    fn test_provider_factory_default() {
        // Test default provider creation
        let provider = ProviderFactory::create_provider();
        assert_eq!(provider.get_provider_name(), "OpenAI");
    }

    #[test]
    #[serial]
    fn test_provider_factory_with_env_var() {
        // Test with environment variable
        unsafe { env::set_var(LLM_PROVIDER_ENV, PROVIDER_OPENAI_GPT4); }
        let provider = ProviderFactory::create_provider();
        assert_eq!(provider.get_provider_name(), "OpenAI");
        assert_eq!(provider.get_config().model, GPT4_MODEL);
        unsafe { env::remove_var(LLM_PROVIDER_ENV); }
        
        // Test with openai
        unsafe { env::set_var(LLM_PROVIDER_ENV, PROVIDER_OPENAI); }
        let provider = ProviderFactory::create_provider();
        assert_eq!(provider.get_provider_name(), "OpenAI");
        assert_eq!(provider.get_config().model, DEFAULT_OPENAI_MODEL);
        unsafe { env::remove_var(LLM_PROVIDER_ENV); }
        
        // Test with unknown provider (should default to OpenAI)
        unsafe { env::set_var(LLM_PROVIDER_ENV, "unknown"); }
        let provider = ProviderFactory::create_provider();
        assert_eq!(provider.get_provider_name(), "OpenAI");
        unsafe { env::remove_var(LLM_PROVIDER_ENV); }
    }

    #[test]
    fn test_provider_factory_specific_methods() {
        let provider = ProviderFactory::create_openai();
        assert_eq!(provider.get_provider_name(), "OpenAI");
        assert_eq!(provider.get_config().model, DEFAULT_OPENAI_MODEL);
        
        let provider = ProviderFactory::create_openai_with_model("gpt-4-turbo");
        assert_eq!(provider.get_provider_name(), "OpenAI");
        assert_eq!(provider.get_config().model, "gpt-4-turbo");
    }

    #[test]
    #[serial]
    fn test_api_key_retrieval_failure() {
        let provider = MockProvider::new();
        
        unsafe { env::remove_var(MOCK_API_KEY_ENV); }
        let result = provider.get_api_key();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains(&format!("{} environment variable not set", MOCK_API_KEY_ENV)));
    }

    #[test]
    #[serial]
    fn test_api_key_retrieval_success() {
        let provider = MockProvider::new();
        
        unsafe { env::set_var(MOCK_API_KEY_ENV, "test_api_key"); }
        let result = provider.get_api_key();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_api_key");
        unsafe { env::remove_var(MOCK_API_KEY_ENV); }
    }
}