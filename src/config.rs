use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

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

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(config_path)?;

    writeln!(file, "export {key}=\"{value}\"")?;
    println!("Set {key} in {}", config_path.display());
    Ok(())
}

/// Show environment variables from config file
pub fn show_config(config_path: &Path) -> io::Result<()> {
    if config_path.exists() {
        let content = fs::read_to_string(config_path)?;
        println!("--- .committorc content ---");
        let re = regex::Regex::new(r#"^export\s+([A-Z_]+)="(.*)"$"#).unwrap();
        for line in content.lines() {
            if let Some(caps) = re.captures(line) {
                println!("{}: {}", &caps[1], &caps[2]);
            } else {
                println!("{line}"); // Print lines that don't match the pattern as is
            }
        }
    } else {
        println!("No .committorc file found at {}.", config_path.display());
    }
    Ok(())
}