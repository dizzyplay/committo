use async_trait::async_trait;
use committo::api::{LlmConfig, LlmError, LlmProvider};
use committo::config::ConfigProvider;

/// Mock config for testing
#[derive(Debug, Clone)]
pub struct MockConfig {
    pub api_key: Option<String>,
    pub candidate_count: Option<u32>,
    pub dev_mode: Option<bool>,
}

impl Default for MockConfig {
    fn default() -> Self {
        Self {
            api_key: Some("test_key".to_string()),
            candidate_count: Some(1),
            dev_mode: Some(false),
        }
    }
}

impl ConfigProvider for MockConfig {
    fn get_api_key(&self) -> Option<String> {
        self.api_key.clone()
    }
    
    fn get_llm_provider(&self) -> Option<String> {
        Some("mock".to_string())
    }
    
    fn get_llm_model(&self) -> Option<String> {
        Some("mock-model".to_string())
    }
    
    fn get_candidate_count(&self) -> Option<u32> {
        self.candidate_count
    }
    
    fn get_dev_mode(&self) -> Option<bool> {
        self.dev_mode
    }
}

/// Mock provider for testing
pub struct MockProvider {
    config: LlmConfig,
    app_config: Box<dyn ConfigProvider>,
    response: String,
    should_fail: bool,
}

impl MockProvider {
    pub fn new() -> Self {
        Self {
            config: LlmConfig {
                model: "mock-model".to_string(),
                endpoint: "https://mock.api.com/v1/chat/completions".to_string(),
            },
            app_config: Box::new(MockConfig::default()),
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
            app_config: Box::new(MockConfig::default()),
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
            app_config: Box::new(MockConfig::default()),
            response: String::new(),
            should_fail: true,
        }
    }
    
    pub fn with_config(config: MockConfig) -> Self {
        Self {
            config: LlmConfig {
                model: "mock-model".to_string(),
                endpoint: "https://mock.api.com/v1/chat/completions".to_string(),
            },
            app_config: Box::new(config),
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

    fn get_provider_name(&self) -> &'static str {
        "Mock"
    }

    fn get_api_key(&self) -> Result<String, LlmError> {
        self.app_config.get_api_key()
            .filter(|key| !key.is_empty())
            .ok_or_else(|| LlmError::ConfigError("API key not found in config".to_string()))
    }
    
    fn get_candidate_count(&self) -> u32 {
        self.app_config.get_candidate_count().unwrap_or(1)
    }
    
    fn get_dev_mode(&self) -> bool {
        self.app_config.get_dev_mode().unwrap_or(false)
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
}