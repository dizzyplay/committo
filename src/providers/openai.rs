use async_trait::async_trait;
use crate::api::{LlmConfig, LlmError, LlmProvider};

/// OpenAI provider implementation
pub struct OpenAiProvider {
    config: LlmConfig,
}

impl OpenAiProvider {
    pub fn new() -> Self {
        Self {
            config: LlmConfig {
                api_key_env_var: "OPENAI_API".to_string(),
                model: "gpt-3.5-turbo".to_string(),
                endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            },
        }
    }
    
    pub fn with_model(model: &str) -> Self {
        Self {
            config: LlmConfig {
                api_key_env_var: "OPENAI_API".to_string(),
                model: model.to_string(),
                endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            },
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