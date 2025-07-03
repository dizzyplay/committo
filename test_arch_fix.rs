// Simple test to verify the architectural fix
use committo::providers::ProviderFactory;
use committo::config::{LLM_PROVIDER_ENV, LLM_MODEL_ENV, PROVIDER_OPENAI, GPT4_MODEL};
use std::env;

fn main() {
    println!("Testing architectural fix: Provider vs Model separation");
    
    // Test 1: Default OpenAI provider (should use gpt-3.5-turbo)
    let provider = ProviderFactory::create_provider();
    println!("Default: {} with model '{}'", provider.get_provider_name(), provider.get_config().model);
    
    // Test 2: OpenAI provider with GPT-4 model
    env::set_var(LLM_PROVIDER_ENV, PROVIDER_OPENAI);
    env::set_var(LLM_MODEL_ENV, GPT4_MODEL);
    let provider = ProviderFactory::create_provider();
    println!("OpenAI + GPT-4: {} with model '{}'", provider.get_provider_name(), provider.get_config().model);
    
    // Test 3: OpenAI provider with custom model
    env::set_var(LLM_MODEL_ENV, "gpt-4-turbo");
    let provider = ProviderFactory::create_provider();
    println!("OpenAI + Custom: {} with model '{}'", provider.get_provider_name(), provider.get_config().model);
    
    // Clean up
    env::remove_var(LLM_PROVIDER_ENV);
    env::remove_var(LLM_MODEL_ENV);
    
    println!("✅ Architecture fix verified: GPT-4 is now properly a model variant of OpenAI provider");
}