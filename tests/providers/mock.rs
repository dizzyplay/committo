use async_trait::async_trait;
use committo::api::{LlmConfig, LlmError, LlmProvider};

const MOCK_API_KEY_ENV: &str = "MOCK_API_KEY";

/// Mock provider for testing
pub struct MockProvider {
    config: LlmConfig,
    response: String,
    should_fail: bool,
}

impl MockProvider {
    pub fn new() -> Self {
        Self {
            config: LlmConfig {
                api_key_env_var: MOCK_API_KEY_ENV.to_string(),
                model: "mock-model".to_string(),
                endpoint: "https://mock.api.com/v1/chat/completions".to_string(),
            },
            response: "Mock commit message".to_string(),
            should_fail: false,
        }
    }
    
    pub fn with_response(response: &str) -> Self {
        Self {
            config: LlmConfig {
                api_key_env_var: MOCK_API_KEY_ENV.to_string(),
                model: "mock-model".to_string(),
                endpoint: "https://mock.api.com/v1/chat/completions".to_string(),
            },
            response: response.to_string(),
            should_fail: false,
        }
    }
    
    pub fn with_failure() -> Self {
        Self {
            config: LlmConfig {
                api_key_env_var: MOCK_API_KEY_ENV.to_string(),
                model: "mock-model".to_string(),
                endpoint: "https://mock.api.com/v1/chat/completions".to_string(),
            },
            response: String::new(),
            should_fail: true,
        }
    }
}

#[async_trait]
impl LlmProvider for MockProvider {
    fn get_config(&self) -> &LlmConfig {
        &self.config
    }
    
    fn get_provider_name(&self) -> &'static str {
        "Mock"
    }

    async fn generate_commit_message_impl(&self, _system_prompt: &str, _diff: &str) -> Result<String, LlmError> {
        // Check for API key availability (consistent with other providers)
        let _api_key = self.get_api_key()?;
        
        if self.should_fail {
            Err(LlmError::ApiError("Mock API error".to_string()))
        } else {
            Ok(self.response.clone())
        }
    }
}