use async_trait::async_trait;
use std::env;

use crate::convention::find_and_build_prompt;

/// Error type for LLM API operations
#[derive(Debug)]
pub enum LlmError {
    ApiError(String),
    ConfigError(String),
    NetworkError(reqwest::Error),
}

impl From<reqwest::Error> for LlmError {
    fn from(err: reqwest::Error) -> Self {
        LlmError::NetworkError(err)
    }
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LlmError::ApiError(msg) => write!(f, "API Error: {}", msg),
            LlmError::ConfigError(msg) => write!(f, "Configuration Error: {}", msg),
            LlmError::NetworkError(err) => write!(f, "Network Error: {}", err),
        }
    }
}

impl std::error::Error for LlmError {}

/// Trait for different LLM providers
#[async_trait]
pub trait LlmProvider {
    async fn generate_commit_message(&self, diff: &str, dry_run: bool) -> Result<String, LlmError>;
    fn get_api_key_source(&self) -> String;
}

/// OpenAI provider implementation
pub struct OpenAiProvider;

impl OpenAiProvider {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
    async fn generate_commit_message(&self, diff: &str, dry_run: bool) -> Result<String, LlmError> {
        let api_key_source = self.get_api_key_source();

        let guideline = "**IMPORTANT PRIORITY RULES:**\n- Numbers indicate priority: 1 = HIGHEST priority, 2, 3, 4, 5... = lower priority\n- When instructions conflict, ALWAYS follow the higher priority (lower number)\n- Apply these rules when analyzing git diff and generating commit messages\n";
        let custom_conventions = find_and_build_prompt().unwrap_or_default();
        let system_prompt = if custom_conventions.is_empty() {
            "You are an expert at writing git commit messages. Based on the following diff, generate a concise and informative commit message.".to_string()
        } else {
            format!("{}\n{}", guideline, custom_conventions)
        };

        if dry_run {
            println!("--- Dry Run ---");
            println!("API Key Source: {api_key_source}");
            println!("\n--- Prompt ---");
            println!("{system_prompt}");
            println!("\n--- Git Diff ---");
            println!("{diff}");
            println!("--- End Dry Run ---");
            return Ok("Dry run complete.".to_string());
        }

        let api_key = env::var("OPENAI_API")
            .map_err(|_| LlmError::ConfigError("OPENAI_API environment variable not set".to_string()))?;

        let client = reqwest::Client::new();
        let request_body = serde_json::json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": diff}
            ]
        });

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
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


    fn get_api_key_source(&self) -> String {
        match env::var("OPENAI_API") {
            Ok(_) => "Environment variable".to_string(),
            Err(_) => ".committorc file".to_string(),
        }
    }
}

/// Get the default LLM provider
/// TODO: Make this configurable to support Claude, local models, etc.
pub fn get_default_provider() -> Box<dyn LlmProvider + Send + Sync> {
    Box::new(OpenAiProvider::new())
}

/// Generate commit message using the default LLM provider
pub async fn generate_commit_message(diff: &str, dry_run: bool) -> Result<String, LlmError> {
    let provider = get_default_provider();
    provider.generate_commit_message(diff, dry_run).await
}