//! LLM Provider implementations
//! 
//! This module contains implementations for different LLM providers.
//! Each provider is in its own file for better organization and maintainability.

pub mod openai;

// Re-export common provider types
pub use openai::OpenAiProvider;

use crate::api::LlmProvider;
use crate::config::{LLM_PROVIDER_CONFIG, LLM_MODEL_CONFIG, DEFAULT_OPENAI_MODEL, PROVIDER_OPENAI, CONFIG_FILE_NAME};

/// Provider factory for creating LLM providers
pub struct ProviderFactory;

impl ProviderFactory {
    /// Create provider based on config file or default to OpenAI
    pub fn create_provider() -> Box<dyn LlmProvider + Send + Sync> {
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => return Box::new(OpenAiProvider::new()), // Default if no home
        };
        
        let config_path = home_dir.join(CONFIG_FILE_NAME);
        
        let provider_name = crate::config::get_config_value(&config_path, LLM_PROVIDER_CONFIG)
            .ok()
            .flatten()
            .unwrap_or_else(|| PROVIDER_OPENAI.to_string());
            
        let model = crate::config::get_config_value(&config_path, LLM_MODEL_CONFIG)
            .ok()
            .flatten()
            .unwrap_or_else(|| DEFAULT_OPENAI_MODEL.to_string());
        
        match provider_name.as_str() {
            PROVIDER_OPENAI => Box::new(OpenAiProvider::with_model(&model)),
            // Future providers can be added here:
            // "claude" => Box::new(claude::ClaudeProvider::new()),
            // "local" => Box::new(local::LocalLlmProvider::new()),
            _ => Box::new(OpenAiProvider::new()), // Default
        }
    }
    
    /// Create specific OpenAI provider (for dependency injection)
    pub fn create_openai() -> Box<dyn LlmProvider + Send + Sync> {
        Box::new(OpenAiProvider::new())
    }
    
    /// Create OpenAI provider with specific model
    pub fn create_openai_with_model(model: &str) -> Box<dyn LlmProvider + Send + Sync> {
        Box::new(OpenAiProvider::with_model(model))
    }
}

// Future provider modules would be added like this:
// pub mod claude;
// pub mod local;
// pub use claude::ClaudeProvider;
// pub use local::LocalLlmProvider;