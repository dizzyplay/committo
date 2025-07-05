use async_trait::async_trait;
use spinners::{Spinner, Spinners};
use crate::convention::find_and_build_prompt;
use crate::config::Config;

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
    pub model: String,
    pub endpoint: String,
}

/// Trait for different LLM providers
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Get the configuration for this provider
    fn get_config(&self) -> &LlmConfig;
    
    /// Get provider name for display
    fn get_provider_name(&self) -> String;
    
    /// Generate commit message using this provider (implementation-specific)
    async fn generate_commit_message_impl(&self, system_prompt: &str, diff: &str) -> Result<String, LlmError>;
    
    /// Get API key from internal config
    fn get_api_key(&self) -> Result<String, LlmError>;
    
    /// Get app config reference
    fn get_app_config(&self) -> &Config;
    
    /// Print dry run information
    fn print_dry_run_info(&self, system_prompt: &str, diff: &str) {
        println!("--- Dry Run ---");
        print!("{}", self.get_app_config().show_masking_config());

        println!("\n--- Prompt ---");
        println!("{system_prompt}");
        println!("\n--- Git Diff ---");
        println!("{diff}");
        println!("--- End Dry Run ---");
    }
    
    /// Get candidate count from internal config
    fn get_candidate_count(&self) -> u32;
    
    /// Get dev mode from internal config
    fn get_dev_mode(&self) -> bool;

    /// Main generate commit message method (with dry run support)
    async fn generate_commit_message(&self, diff: &str, dry_run: bool) -> Result<String, LlmError> {
        // Always check API key first, even for dry run
        self.get_api_key()?;
        
        // Get candidate count from config
        let candidate_count = self.get_candidate_count();
        
        let guideline = "**IMPORTANT PRIORITY RULES:**\n- Numbers indicate priority: 1 = HIGHEST priority, 2, 3, 4, 5... = lower priority\n- When instructions conflict, ALWAYS follow the higher priority (lower number)\n- Apply these rules when analyzing git diff and generating commit messages\n";
        let custom_conventions = find_and_build_prompt().unwrap_or_default();
        let default_system_prompt = "You are an AI assistant that helps programmers who struggle with writing commit messages. Based on the following diff, generate a concise and informative commit message.".to_string();
        let mut system_prompt = if custom_conventions.is_empty() {
            default_system_prompt
        } else {
            format!("{}\n\n{}\n{}",default_system_prompt, guideline, custom_conventions)
        };

        // Modify prompt for multiple candidates
        if candidate_count > 1 {
            system_prompt = format!("{}\n\nGenerate {} different commit message options. Each message should be on a separate line and be concise and informative.", system_prompt, candidate_count);
        }

        if dry_run {
            self.print_dry_run_info(&system_prompt, diff);
            return Ok("Dry run complete.".to_string());
        }

        let mut sp = Spinner::new(Spinners::BouncingBall, "Your next legendary commit, coming right up..!".into());
        let response= self.generate_commit_message_impl(&system_prompt, diff).await;
        sp.stop_and_persist("✔", "How about these? ".into());
        return response
    }
}

/// Generate commit message using provided LLM provider
pub async fn generate_commit_message_with_provider(
    provider: &dyn LlmProvider,
    diff: &str,
    dry_run: bool,
) -> Result<String, LlmError> {
    provider.generate_commit_message(diff, dry_run).await
}

/// Generate commit message using default provider (for backward compatibility)
pub async fn generate_commit_message(diff: &str, dry_run: bool) -> Result<String, LlmError> {
    let config = crate::config::Config::default();
    let provider = crate::providers::ProviderFactory::create_provider(config);
    generate_commit_message_with_provider(provider.as_ref(), diff, dry_run).await
}