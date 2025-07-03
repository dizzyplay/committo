//! LLM Provider implementations
//! 
//! This module contains implementations for different LLM providers.
//! Each provider is in its own file for better organization and maintainability.

pub mod openai;

pub mod mock;

// Re-export common provider types
pub use openai::OpenAiProvider;

pub use mock::MockProvider;

use std::env;
use crate::api::LlmProvider;

/// Provider factory for creating LLM providers
pub struct ProviderFactory;

impl ProviderFactory {
    /// Create provider based on environment variable or default to OpenAI
    pub fn create_provider() -> Box<dyn LlmProvider + Send + Sync> {
        match env::var("LLM_PROVIDER").as_deref() {
            Ok("openai") => Box::new(OpenAiProvider::new()),
            Ok("openai-gpt4") => Box::new(OpenAiProvider::with_model("gpt-4")),
            // Future providers can be added here:
            // Ok("claude") => Box::new(claude::ClaudeProvider::new()),
            // Ok("local") => Box::new(local::LocalLlmProvider::new()),
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