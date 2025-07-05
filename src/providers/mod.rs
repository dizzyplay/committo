//! LLM Provider implementations
//! 
//! This module contains implementations for different LLM providers.
//! Each provider is in its own file for better organization and maintainability.

pub mod openai;

// Re-export common provider types
pub use openai::OpenAiProvider;

use crate::api::LlmProvider;
use crate::config::{DEFAULT_OPENAI_MODEL, PROVIDER_OPENAI, CONFIG_FILE_NAME, Config, ConfigProvider, load_config};

/// Provider factory for creating LLM providers
pub struct ProviderFactory;

impl ProviderFactory {
    /// Create provider based on config file or default to OpenAI
    pub fn create_provider() -> Box<dyn LlmProvider + Send + Sync> {
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => {
                let config = Config::default();
                return Box::new(OpenAiProvider::new(Box::new(config)));
            }
        };
        
        let config_path = home_dir.join(CONFIG_FILE_NAME);
        let config = load_config(&config_path).unwrap_or_default();
        
        let provider_name = config.get_llm_provider().unwrap_or_else(|| PROVIDER_OPENAI.to_string());
        let model = config.get_llm_model().unwrap_or_else(|| DEFAULT_OPENAI_MODEL.to_string());
        
        match provider_name.as_str() {
            PROVIDER_OPENAI => Box::new(OpenAiProvider::with_model(Box::new(config), &model)),
            // Future providers can be added here:
            // "claude" => Box::new(claude::ClaudeProvider::new()),
            // "local" => Box::new(local::LocalLlmProvider::new()),
            _ => Box::new(OpenAiProvider::new(Box::new(config))), // Default
        }
    }
    
    /// Create specific OpenAI provider (for dependency injection)
    pub fn create_openai() -> Box<dyn LlmProvider + Send + Sync> {
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => {
                let config = Config::default();
                return Box::new(OpenAiProvider::new(Box::new(config)));
            }
        };
        
        let config_path = home_dir.join(CONFIG_FILE_NAME);
        let config = load_config(&config_path).unwrap_or_default();
        Box::new(OpenAiProvider::new(Box::new(config)))
    }
    
    /// Create OpenAI provider with specific model
    pub fn create_openai_with_model(model: &str) -> Box<dyn LlmProvider + Send + Sync> {
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => {
                let config = Config::default();
                return Box::new(OpenAiProvider::with_model(Box::new(config), model));
            }
        };
        
        let config_path = home_dir.join(CONFIG_FILE_NAME);
        let config = load_config(&config_path).unwrap_or_default();
        Box::new(OpenAiProvider::with_model(Box::new(config), model))
    }
}

// Future provider modules would be added like this:
// pub mod claude;
// pub mod local;
// pub use claude::ClaudeProvider;
// pub use local::LocalLlmProvider;