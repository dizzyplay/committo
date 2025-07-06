use async_trait::async_trait;
use committo::config::Config;
use committo::api::{LlmConfig, LlmError, LlmProvider};

/// Mock provider for testing
pub struct MockProvider {
    config: LlmConfig,
    app_config: Config,
    response: String,
    should_fail: bool,
}

impl MockProvider {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            config: LlmConfig {
                model: "mock-model".to_string(),
                endpoint: "https://mock.api.com/v1/chat/completions".to_string(),
            },
            app_config: Config {
                api_key: Some("test_key".to_string()),
                candidate_count: Some(1),
                llm_provider: Some("mock".to_string()),
                llm_model: Some("mock-model".to_string()),
            },
            response: "Mock commit message".to_string(),
            should_fail: false,
        }
    }
    
    pub fn with_response(response: &str) -> Self {
        Self {
            config: LlmConfig {
                model: "mock-model".to_string(),
                endpoint: "https://mock.api.com/v1/chat/completions".to_string(),
            },
            app_config: Config {
                api_key: Some("test_key".to_string()),
                candidate_count: Some(1),
                llm_provider: Some("mock".to_string()),
                llm_model: Some("mock-model".to_string()),
            },
            response: response.to_string(),
            should_fail: false,
        }
    }
    
    pub fn with_failure() -> Self {
        Self {
            config: LlmConfig {
                model: "mock-model".to_string(),
                endpoint: "https://mock.api.com/v1/chat/completions".to_string(),
            },
            app_config: Config {
                api_key: Some("test_key".to_string()),
                candidate_count: Some(1),
                llm_provider: Some("mock".to_string()),
                llm_model: Some("mock-model".to_string()),
            },
            response: String::new(),
            should_fail: true,
        }
    }
    
    #[allow(dead_code)]
    pub fn with_config(config: Config) -> Self {
        Self {
            config: LlmConfig {
                model: "mock-model".to_string(),
                endpoint: "https://mock.api.com/v1/chat/completions".to_string(),
            },
            app_config: config,
            response: "Mock commit message".to_string(),
            should_fail: false,
        }
    }
}

#[async_trait]
impl LlmProvider for MockProvider {
    fn get_config(&self) -> &LlmConfig {
        &self.config
    }
    
    fn get_provider_name(&self) -> String {
        self.app_config.llm_provider.clone().unwrap_or_else(|| "Mock".to_string())
    }

    async fn generate_commit_message_impl(&self, system_prompt: &str, _diff: &str) -> Result<String, LlmError> {
        // Check for API key availability (consistent with other providers)
        let _api_key = self.get_api_key()?;

        if self.should_fail {
            Err(LlmError::ApiError("Mock API error".to_string()))
        } else {
            // Check if multiple candidates are requested based on the system prompt
            if system_prompt.contains("different commit message options") {
                // Extract the number from the prompt
                if let Some(count_str) = system_prompt.split("Generate ").nth(1).and_then(|s| s.split(" different").next()) {
                    if let Ok(count) = count_str.parse::<u32>() {
                        let mut candidates = Vec::new();
                        for i in 1..=count {
                            candidates.push(format!("{} #{}", self.response, i));
                        }
                        return Ok(candidates.join("\n"));
                    }
                }
            }
            Ok(self.response.clone())
        }
    }

    fn get_api_key(&self) -> Result<String, LlmError> {
        self.app_config.api_key.clone()
            .filter(|key| !key.is_empty())
            .ok_or_else(|| LlmError::ConfigError("API key not found in config".to_string()))
    }

    fn get_candidate_count(&self) -> u32 {
        self.app_config.candidate_count.unwrap_or(1)
    }


    fn get_app_config(&self) -> &Config {
        &self.app_config
    }
}