//! LLM Provider implementations
//! 
//! This module contains implementations for different LLM providers.
//! Each provider is in its own file for better organization and maintainability.

pub mod openai;

// Re-export common provider types
pub use openai::OpenAiProvider;

use crate::api::LlmProvider;
use crate::config::{DEFAULT_OPENAI_MODEL, PROVIDER_OPENAI, Config};

/// Provider factory for creating LLM providers
pub struct ProviderFactory;

impl ProviderFactory {
    /// Create provider based on provided config
    pub fn create_provider(config: Config) -> Box<dyn LlmProvider + Send + Sync> {
        let provider_name = config.llm_provider.clone().unwrap_or_else(|| PROVIDER_OPENAI.to_string());
        let model = config.llm_model.clone().unwrap_or_else(|| DEFAULT_OPENAI_MODEL.to_string());
        
        match provider_name.as_str() {
            PROVIDER_OPENAI => Box::new(OpenAiProvider::with_model(config, &model)),
            // Future providers can be added here:
            // "claude" => Box::new(claude::ClaudeProvider::new()),
            // "local" => Box::new(local::LocalLlmProvider::new()),
            _ => Box::new(OpenAiProvider::new(config)), // Default
        }
    }
    
    /// Create specific OpenAI provider with config
    pub fn create_openai(config: Config) -> Box<dyn LlmProvider + Send + Sync> {
        Box::new(OpenAiProvider::new(config))
    }
    
    /// Create OpenAI provider with specific model and config
    pub fn create_openai_with_model(config: Config, model: &str) -> Box<dyn LlmProvider + Send + Sync> {
        Box::new(OpenAiProvider::with_model(config, model))
    }
}

// Future provider modules would be added like this:
// pub mod claude;
// pub mod local;
// pub use claude::ClaudeProvider;
// pub use local::LocalLlmProvider;