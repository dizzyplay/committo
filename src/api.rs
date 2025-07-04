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

/// Configuration for LLM providers
#[derive(Clone)]
pub struct LlmConfig {
    pub api_key_env_var: String,
    pub model: String,
    pub endpoint: String,
}

/// Trait for different LLM providers
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Get the configuration for this provider
    fn get_config(&self) -> &LlmConfig;
    
    /// Get provider name for display
    fn get_provider_name(&self) -> &'static str;
    
    /// Generate commit message using this provider (implementation-specific)
    async fn generate_commit_message_impl(&self, system_prompt: &str, diff: &str) -> Result<String, LlmError>;
    
    /// Get API key from environment or config file
    fn get_api_key(&self) -> Result<String, LlmError> {
        env::var(&self.get_config().api_key_env_var)
            .map_err(|_| LlmError::ConfigError(format!("{} environment variable not set", self.get_config().api_key_env_var)))
    }
    
    /// Get API key source description
    fn get_api_key_source(&self) -> String {
        match env::var(&self.get_config().api_key_env_var) {
            Ok(_) => "Environment variable".to_string(),
            Err(_) => format!("{} file", crate::config::CONFIG_FILE_NAME),
        }
    }
    
    /// Mask API key for display (first 5 chars + asterisks)
    fn mask_api_key(&self, api_key: &str) -> String {
        if api_key.len() >= 5 {
            format!("{}{}", &api_key[..5], "*".repeat(api_key.len() - 5))
        } else {
            "*".repeat(api_key.len())
        }
    }
    
    /// Print dry run information
    fn print_dry_run_info(&self, system_prompt: &str, diff: &str) {
        println!("--- Dry Run ---");
        println!("Provider: {}", self.get_provider_name());
        println!("API Key Source: {}", self.get_api_key_source());
        
        if let Ok(api_key) = env::var(&self.get_config().api_key_env_var) {
            println!("API Key: {}", self.mask_api_key(&api_key));
        }
        
        println!("\n--- Prompt ---");
        println!("{system_prompt}");
        println!("\n--- Git Diff ---");
        println!("{diff}");
        println!("--- End Dry Run ---");
    }
    
    /// Main generate commit message method (with dry run support)
    async fn generate_commit_message(&self, diff: &str, dry_run: bool, candidate_count: u32) -> Result<String, LlmError> {
        // Always check API key first, even for dry run
        self.get_api_key()?;
        
        let guideline = "**IMPORTANT PRIORITY RULES:**\n- Numbers indicate priority: 1 = HIGHEST priority, 2, 3, 4, 5... = lower priority\n- When instructions conflict, ALWAYS follow the higher priority (lower number)\n- Apply these rules when analyzing git diff and generating commit messages\n";
        let custom_conventions = find_and_build_prompt().unwrap_or_default();
        let mut system_prompt = if custom_conventions.is_empty() {
            "You are an expert at writing git commit messages. Based on the following diff, generate a concise and informative commit message.".to_string()
        } else {
            format!("{}\n{}", guideline, custom_conventions)
        };

        // Modify prompt for multiple candidates
        if candidate_count > 1 {
            system_prompt = format!("{}\n\nGenerate {} different commit message options. Each message should be on a separate line and be concise and informative.", system_prompt, candidate_count);
        }

        if dry_run {
            self.print_dry_run_info(&system_prompt, diff);
            return Ok("Dry run complete.".to_string());
        }

        self.generate_commit_message_impl(&system_prompt, diff).await
    }
}

/// Generate commit message using provided LLM provider
pub async fn generate_commit_message_with_provider(
    provider: &dyn LlmProvider,
    diff: &str,
    dry_run: bool,
    candidate_count: u32,
) -> Result<String, LlmError> {
    provider.generate_commit_message(diff, dry_run, candidate_count).await
}

/// Generate commit message using default provider (for backward compatibility)
pub async fn generate_commit_message(diff: &str, dry_run: bool) -> Result<String, LlmError> {
    let provider = crate::providers::ProviderFactory::create_provider();
    generate_commit_message_with_provider(provider.as_ref(), diff, dry_run, 1).await
}