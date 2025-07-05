use async_trait::async_trait;
use crate::api::{LlmConfig, LlmError, LlmProvider};
use crate::config::{DEFAULT_OPENAI_MODEL, ConfigProvider};

/// OpenAI provider implementation
pub struct OpenAiProvider {
    config: LlmConfig,
    app_config: Box<dyn ConfigProvider>,
}

impl OpenAiProvider {
    pub fn new(app_config: Box<dyn ConfigProvider>) -> Self {
        Self {
            config: LlmConfig {
                model: DEFAULT_OPENAI_MODEL.to_string(),
                endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            },
            app_config,
        }
    }
    
    pub fn with_model(app_config: Box<dyn ConfigProvider>, model: &str) -> Self {
        Self {
            config: LlmConfig {
                model: model.to_string(),
                endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            },
            app_config,
        }
    }
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
    fn get_config(&self) -> &LlmConfig {
        &self.config
    }
    
    fn get_provider_name(&self) -> &'static str {
        "OpenAI"
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

    async fn generate_commit_message_impl(&self, system_prompt: &str, diff: &str) -> Result<String, LlmError> {
        let api_key = self.get_api_key()?;
        let client = reqwest::Client::new();
        
        let request_body = serde_json::json!({
            "model": self.config.model,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": diff}
            ]
        });

        let response = client
            .post(&self.config.endpoint)
            .bearer_auth(api_key)
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(format!(
                "API request failed with status: {}",
                response.status()
            )));
        }

        let response_data: serde_json::Value = response.json().await?;
        
        let content = response_data
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
            .ok_or_else(|| LlmError::ApiError("Invalid response format from OpenAI API".to_string()))?;

        Ok(content.to_string())
    }
}