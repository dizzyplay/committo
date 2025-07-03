use clap::{Parser, Subcommand};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// Commit message generator powered by LLM
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Manage environment variables
    Env {
        #[command(subcommand)]
        command: EnvCommands,
    },
    /// Generate a commit message
    Generate {
        /// Run in dry-run mode to see the generated prompt without calling the API
        #[arg(long)]
        dry_run: bool,
    },
    /// Quick dry-run (alias for generate --dry-run)
    Dev,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum EnvCommands {
    /// Set an environment variable, e.g., OPENAI_API='key'
    Set {
        /// The environment variable to set, e.g., OPENAI_API='key'
        #[arg(value_name = "KEY_VALUE_PAIR")]
        pair: String,
    },
    /// Show current environment variables
    Show {},
}

#[derive(Serialize)]
struct ChatCompletionRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

fn handle_set_command(pair: &str, config_path: &Path) -> io::Result<()> {
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

    writeln!(file, "export {key}=\"{value}\"
")?;
    println!("Set {key} in {}", config_path.display());
    Ok(())
}

use std::fs;

fn find_and_build_prompt() -> io::Result<String> {
    let mut prompt_parts = Vec::new();
    let current_dir = env::current_dir()?;

    for ancestor in current_dir.ancestors() {
        let convention_path = ancestor.join(".committoconvention");
        if convention_path.exists() {
            let content = fs::read_to_string(convention_path)?;
            prompt_parts.push(content);
        }
    }

    // The prompts are found from child to parent, so we reverse to get parent-to-child order.
    prompt_parts.reverse();
    Ok(prompt_parts.join("\n\n"))
}

async fn generate_commit_message(diff: &str, dry_run: bool) -> Result<String, reqwest::Error> {
    let api_key_source = match env::var("OPENAI_API") {
        Ok(_) => "Environment variable",
        Err(_) => ".committorc file",
    };

    let guideline = "The importance of these guidelines increases from top to bottom. Please consider this when analyzing the git diff and generate appropriate commit messages accordingly.";
    let custom_conventions = find_and_build_prompt().unwrap_or_default();
    let system_prompt = if custom_conventions.is_empty() {
        "You are an expert at writing git commit messages. Based on the following diff, generate a concise and informative commit message.".to_string()
    } else {
        format!("{}\n{}", custom_conventions, guideline)
    };

    if dry_run {
        println!("--- Dry Run ---");
        println!("API Key Source: {api_key_source}");
        println!("\n--- Prompt ---");
        println!("{system_prompt}");
        println!("\n--- Git Diff ---");
        println!("{diff}");
        println!("--- End Dry Run ---");
        return Ok("Dry run complete.".to_string());
    }
    
    let api_key = env::var("OPENAI_API").expect("OPENAI_API must be set.");
    let client = Client::new();

    let request_body = ChatCompletionRequest {
        model: "gpt-3.5-turbo",
        messages: vec![
            Message { role: "system", content: &system_prompt },
            Message { role: "user", content: diff },
        ],
    };

    let res = client.post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await?;

    let response_body: ChatCompletionResponse = res.json().await?;
    Ok(response_body.choices[0].message.content.clone())
}

pub async fn run(cli: Cli) -> io::Result<()> {
    match cli.command {
        Commands::Env { command } => {
            let home_path = home::home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Cannot find home directory"))?;
            let config_path = home_path.join(".committorc");

            match command {
                EnvCommands::Set { pair } => {
                    handle_set_command(&pair, &config_path)?;
                }
                EnvCommands::Show {} => {
                    if config_path.exists() {
                        let content = fs::read_to_string(&config_path)?;
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
                }
            }
        }
        Commands::Generate { dry_run } => {
            // Check for COMMITTO_DEV environment variable to force dry-run
            let force_dry_run = env::var("COMMITTO_DEV").is_ok();
            let effective_dry_run = dry_run || force_dry_run;
            
            let home_path = home::home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Cannot find home directory"))?;
            let config_path = home_path.join(".committorc");
            if config_path.exists() {
                dotenvy::from_path(&config_path).ok();
            }

            let output = Command::new("git")
                .arg("diff")
                .arg("--staged")
                .output()?;

            if !output.status.success() {
                eprintln!("Error: Failed to execute git diff --staged");
                eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }

            let diff = String::from_utf8_lossy(&output.stdout);
            if !effective_dry_run && diff.trim().is_empty() {
                println!("No staged changes to commit.");
                return Ok(());
            }

            let commit_message = generate_commit_message(&diff, effective_dry_run).await.map_err(io::Error::other)?;
            println!("{commit_message}");
        }
        Commands::Dev => {
            // Dev command is just an alias for generate --dry-run
            let generate_cmd = Commands::Generate { dry_run: true };
            return Box::pin(run(Cli { command: generate_cmd })).await;
        }
    }
    Ok(())
}
