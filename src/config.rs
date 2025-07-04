use std::fs;
use std::io;
use std::path::Path;
use serde::{Deserialize, Serialize};

/// Configuration file name
pub const CONFIG_FILE_NAME: &str = "committo.toml";

/// Convention file name  
pub const CONVENTION_FILE_NAME: &str = ".committoconvention";

/// Configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(rename = "api-key")]
    pub api_key: Option<String>,
    
    #[serde(rename = "candidate-count")]
    pub candidate_count: Option<u32>,
    
    #[serde(rename = "llm-provider")]
    pub llm_provider: Option<String>,
    
    #[serde(rename = "llm-model")]
    pub llm_model: Option<String>,
    
    #[serde(rename = "committo-dev")]
    pub committo_dev: Option<bool>,
}

/// Config keys for TOML file
pub const API_KEY_CONFIG: &str = "api-key";
pub const LLM_PROVIDER_CONFIG: &str = "llm-provider";
pub const LLM_MODEL_CONFIG: &str = "llm-model";
pub const COMMITTO_DEV_CONFIG: &str = "committo-dev";
pub const CANDIDATE_COUNT_CONFIG: &str = "candidate-count";

/// Default OpenAI models
pub const DEFAULT_OPENAI_MODEL: &str = "gpt-3.5-turbo";
pub const GPT4_MODEL: &str = "gpt-4";

/// Provider identifiers for LLM_PROVIDER environment variable
pub const PROVIDER_OPENAI: &str = "openai";

/// Load config from TOML file
pub fn load_config(config_path: &Path) -> io::Result<Config> {
    if !config_path.exists() {
        return Ok(Config::default());
    }
    
    let content = fs::read_to_string(config_path)?;
    toml::from_str(&content).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse TOML: {}", e))
    })
}

/// Save config to TOML file
pub fn save_config(config: &Config, config_path: &Path) -> io::Result<()> {
    let toml_string = toml::to_string_pretty(config).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("Failed to serialize TOML: {}", e))
    })?;
    
    fs::write(config_path, toml_string)?;
    Ok(())
}

/// Handle setting config values
pub fn handle_set_command(key: &str, value: &str, config_path: &Path) -> io::Result<()> {
    let mut config = load_config(config_path)?;
    
    match key {
        API_KEY_CONFIG => config.api_key = Some(value.to_string()),
        CANDIDATE_COUNT_CONFIG => {
            let count: u32 = value.parse().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidInput, "candidate-count must be a number")
            })?;
            config.candidate_count = Some(count);
        },
        LLM_PROVIDER_CONFIG => config.llm_provider = Some(value.to_string()),
        LLM_MODEL_CONFIG => config.llm_model = Some(value.to_string()),
        COMMITTO_DEV_CONFIG => {
            let dev: bool = value.parse().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidInput, "committo-dev must be true or false")
            })?;
            config.committo_dev = Some(dev);
        },
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid config key '{}'. Valid keys are: api-key, candidate-count, llm-provider, llm-model, committo-dev", key),
            ));
        }
    }
    
    save_config(&config, config_path)?;
    println!("Set {} = {} in {}", key, value, config_path.display());
    Ok(())
}

/// Show config values from TOML file
pub fn show_config(config_path: &Path) -> io::Result<()> {
    if config_path.exists() {
        let config = load_config(config_path)?;
        println!("--- {} content ---", CONFIG_FILE_NAME);
        
        if let Some(api_key) = &config.api_key {
            println!("api-key = \"{}\"", api_key);
        }
        if let Some(count) = config.candidate_count {
            println!("candidate-count = {}", count);
        }
        if let Some(provider) = &config.llm_provider {
            println!("llm-provider = \"{}\"", provider);
        }
        if let Some(model) = &config.llm_model {
            println!("llm-model = \"{}\"", model);
        }
        if let Some(dev) = config.committo_dev {
            println!("committo-dev = {}", dev);
        }
    } else {
        println!("No {} file found at {}.", CONFIG_FILE_NAME, config_path.display());
    }
    Ok(())
}

/// Get specific config value
pub fn get_config_value(config_path: &Path, key: &str) -> io::Result<Option<String>> {
    let config = load_config(config_path)?;
    
    let value = match key {
        API_KEY_CONFIG => config.api_key,
        CANDIDATE_COUNT_CONFIG => config.candidate_count.map(|v| v.to_string()),
        LLM_PROVIDER_CONFIG => config.llm_provider,
        LLM_MODEL_CONFIG => config.llm_model,
        COMMITTO_DEV_CONFIG => config.committo_dev.map(|v| v.to_string()),
        _ => None,
    };
    
    Ok(value)
}