use std::fs;
use std::io;
use std::path::Path;

/// Configuration file name
pub const CONFIG_FILE_NAME: &str = ".committorc";

/// Convention file name  
pub const CONVENTION_FILE_NAME: &str = ".committoconvention";

/// Environment variable names
pub const OPENAI_API_KEY_ENV: &str = "OPENAI_API_KEY";
pub const LLM_PROVIDER_ENV: &str = "LLM_PROVIDER";
pub const LLM_MODEL_ENV: &str = "LLM_MODEL";
pub const COMMITTO_DEV_ENV: &str = "COMMITTO_DEV";
pub const CANDIDATE_COUNT_ENV: &str = "CANDIDATE_COUNT";

/// Default OpenAI models
pub const DEFAULT_OPENAI_MODEL: &str = "gpt-3.5-turbo";
pub const GPT4_MODEL: &str = "gpt-4";

/// Provider identifiers for LLM_PROVIDER environment variable
pub const PROVIDER_OPENAI: &str = "openai";

/// Handle setting environment variables in config file
pub fn handle_set_command(pair: &str, config_path: &Path) -> io::Result<()> {
    let parts: Vec<&str> = pair.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid format. Please use KEY='value'.",
        ));
    }
    let key = parts[0];
    let value = parts[1].trim_matches(|c| c == '\'' || c == '"');

    // Read existing config if it exists
    let mut lines = Vec::new();
    let mut key_found = false;
    
    if config_path.exists() {
        let content = fs::read_to_string(config_path)?;
        let re = regex::Regex::new(r#"^export\s+([A-Z_]+)=".*"$"#).unwrap();
        
        for line in content.lines() {
            if let Some(caps) = re.captures(line) {
                if &caps[1] == key {
                    // Update existing key
                    lines.push(format!("export {key}=\"{value}\""));
                    key_found = true;
                } else {
                    // Keep other keys unchanged
                    lines.push(line.to_string());
                }
            } else {
                // Keep non-export lines unchanged
                lines.push(line.to_string());
            }
        }
    }
    
    // If key wasn't found, add it
    if !key_found {
        lines.push(format!("export {key}=\"{value}\""));
    }
    
    // Write all lines back to file
    fs::write(config_path, lines.join("\n") + "\n")?;
    
    if key_found {
        println!("Updated {key} in {}", config_path.display());
    } else {
        println!("Set {key} in {}", config_path.display());
    }
    
    Ok(())
}

/// Show environment variables from config file
pub fn show_config(config_path: &Path) -> io::Result<()> {
    if config_path.exists() {
        let content = fs::read_to_string(config_path)?;
        println!("--- {} content ---", CONFIG_FILE_NAME);
        let re = regex::Regex::new(r#"^export\s+([A-Z_]+)="(.*)"$"#).unwrap();
        for line in content.lines() {
            if let Some(caps) = re.captures(line) {
                println!("{}: {}", &caps[1], &caps[2]);
            } else {
                println!("{line}"); // Print lines that don't match the pattern as is
            }
        }
    } else {
        println!("No {} file found at {}.", CONFIG_FILE_NAME, config_path.display());
    }
    Ok(())
}