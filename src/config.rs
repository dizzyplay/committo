use std::fs;
use std::io;
use std::path::Path;
use serde::{Deserialize, Serialize};

/// Configuration file name
pub const CONFIG_FILE_NAME: &str = "committo.toml";

/// Convention file name  
pub const CONVENTION_FILE_NAME: &str = ".committoconvention";

/// Trait for providing configuration to LLM providers
pub trait ConfigProvider: Send + Sync {
    /// Get API key
    fn get_api_key(&self) -> Option<String>;
    
    /// Get LLM provider name
    fn get_llm_provider(&self) -> Option<String>;
    
    /// Get LLM model
    fn get_llm_model(&self) -> Option<String>;
    
    /// Get candidate count
    fn get_candidate_count(&self) -> Option<u32>;
    
    /// Get dev mode (dry run)
    fn get_dev_mode(&self) -> Option<bool>;
}

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

impl ConfigProvider for Config {
    fn get_api_key(&self) -> Option<String> {
        self.api_key.clone()
    }
    
    fn get_llm_provider(&self) -> Option<String> {
        self.llm_provider.clone()
    }
    
    fn get_llm_model(&self) -> Option<String> {
        self.llm_model.clone()
    }
    
    fn get_candidate_count(&self) -> Option<u32> {
        self.candidate_count
    }
    
    fn get_dev_mode(&self) -> Option<bool> {
        self.committo_dev
    }
}

impl Config {
    /// Create new config instance, loading from file or creating interactively if needed
    pub fn new(config_path: &Path) -> io::Result<(Config, std::path::PathBuf)> {
        let config_path_buf = config_path.to_path_buf();
        
        let config = if config_path.exists() {
            Config::load(config_path)?
        } else {
            println!("No configuration file found at: {}", config_path.display());
            println!("Let's set up your configuration interactively!");
            
            Config::interactive_setup(config_path)?
        };
        
        Ok((config, config_path_buf))
    }

    /// Load config from TOML file (assumes file exists)
    fn load(config_path: &Path) -> io::Result<Config> {
        let content = fs::read_to_string(config_path)?;
        toml::from_str(&content).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse TOML: {}", e))
        })
    }

    /// Save config to TOML file
    pub fn save(&self, config_path: &Path) -> io::Result<()> {
        let toml_string = toml::to_string_pretty(self).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Failed to serialize TOML: {}", e))
        })?;
        
        fs::write(config_path, toml_string)?;
        Ok(())
    }

    /// Set a config value
    pub fn set_value(&mut self, key: &str, value: &str) -> io::Result<()> {
        match key {
            API_KEY_CONFIG => self.api_key = Some(value.to_string()),
            CANDIDATE_COUNT_CONFIG => {
                let count: u32 = value.parse().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidInput, "candidate-count must be a number")
                })?;
                self.candidate_count = Some(count);
            },
            LLM_PROVIDER_CONFIG => self.llm_provider = Some(value.to_string()),
            LLM_MODEL_CONFIG => self.llm_model = Some(value.to_string()),
            COMMITTO_DEV_CONFIG => {
                let dev: bool = value.parse().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidInput, "committo-dev must be true or false")
                })?;
                self.committo_dev = Some(dev);
            },
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Invalid config key '{}'. Valid keys are: api-key, candidate-count, llm-provider, llm-model, committo-dev", key),
                ));
            }
        }
        Ok(())
    }

    /// Handle setting config values and save
    pub fn set_and_save(&mut self, key: &str, value: &str, config_path: &Path) -> io::Result<()> {
        self.set_value(key, value)?;
        self.save(config_path)?;
        println!("Set {} = {} in {}", key, value, config_path.display());
        Ok(())
    }

    /// Show config values
    pub fn show(&self) -> io::Result<()> {
        print!("{}", self.show_masking_config());
        Ok(())
    }

    /// Static method for handling set command (for backwards compatibility)
    pub fn handle_set_command(key: &str, value: &str, config_path: &Path) -> io::Result<()> {
        let mut config = if config_path.exists() {
            Config::load(config_path)?
        } else {
            Config::default()
        };
        config.set_and_save(key, value, config_path)
    }

    /// Interactive configuration setup
    fn interactive_setup(config_path: &Path) -> io::Result<Config> {
        use dialoguer::{Input, Select, Confirm};
        
        println!("\n=== Committo Configuration Setup ===");
        
        // API Key setup
        let api_key: String = Input::new()
            .with_prompt("Enter your OpenAI API key")
            .interact_text()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
        // Provider selection
        let providers = vec!["openai"];
        let provider_selection = Select::new()
            .with_prompt("Select LLM provider")
            .items(&providers)
            .default(0)
            .interact()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
        // Model selection
        let models = match providers[provider_selection] {
            "openai" => vec!["gpt-3.5-turbo", "gpt-4", "gpt-4.1-mini-2025-04-14"],
            _ => vec!["gpt-3.5-turbo"],
        };
        
        let model_selection = Select::new()
            .with_prompt("Select model")
            .items(&models)
            .default(0)
            .interact()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
        // Candidate count
        let candidate_count: u32 = Input::new()
            .with_prompt("Number of commit message candidates")
            .default(5)
            .interact_text()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
        // Dev mode
        let dev_mode = Confirm::new()
            .with_prompt("Enable development mode (dry-run by default)?")
            .default(false)
            .interact()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
        let config = Config {
            api_key: Some(api_key),
            llm_provider: Some(providers[provider_selection].to_string()),
            llm_model: Some(models[model_selection].to_string()),
            candidate_count: Some(candidate_count),
            committo_dev: Some(dev_mode),
        };
        
        // Create parent directory if needed
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        config.save(config_path)?;
        
        println!("\n✅ Configuration saved to: {}", config_path.display());
        println!("You can modify it later using 'committo set <key> <value>' or by editing the file directly.");
        
        Ok(config)
    }
    
    /// Mask API key for secure display
    pub fn mask_api_key(&self, api_key: &str) -> String {
        if api_key.len() >= 5 {
            format!("{}{}", &api_key[..5], "*".repeat(api_key.len() - 5))
        } else {
            "*".repeat(api_key.len())
        }
    }
    
    /// Show config in dry run format
    pub fn show_masking_config(&self) -> String {
        let mut output = String::new();
        output.push_str("--- Configuration ---\n");
        
        if let Some(api_key) = &self.api_key {
            output.push_str(&format!("Api Key : \"{}\" (masked)\n", self.mask_api_key(api_key)));
        }
        if let Some(count) = self.candidate_count {
            output.push_str(&format!("Candidate Count : {}\n", count));
        }
        if let Some(provider) = &self.llm_provider {
            output.push_str(&format!("LLM Provider : \"{}\"\n", provider));
        }
        if let Some(model) = &self.llm_model {
            output.push_str(&format!("LLM Model : \"{}\"\n", model));
        }
        if let Some(dev) = self.committo_dev {
            output.push_str(&format!("Dev : {}\n", dev));
        }
        
        output
    }
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

/// Get specific config value
pub fn get_config_value(config_path: &Path, key: &str) -> io::Result<Option<String>> {
    let config = Config::load(config_path)?;
    
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