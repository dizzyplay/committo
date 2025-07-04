pub mod api;
mod cli;
mod commit;
pub mod config;
mod convention;
mod git;
pub mod providers;
mod utils;

pub use cli::{Cli, Commands, EnvCommands};

use std::env;
use std::io;

/// Main application runner
pub async fn run(cli: Cli) -> io::Result<()> {
    match cli.command {
        Commands::Env { command } => {
            let home_path = home::home_dir().ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "Cannot find home directory")
            })?;
            let config_path = home_path.join(config::CONFIG_FILE_NAME);

            match command {
                EnvCommands::Set { pair } => {
                    config::handle_set_command(&pair, &config_path)?;
                }
                EnvCommands::Show {} => {
                    config::show_config(&config_path)?;
                }
            }
        }
        Commands::Generate { dry_run } => {
            // Check for COMMITTO_DEV environment variable to force dry-run
            let force_dry_run = env::var(config::COMMITTO_DEV_ENV).is_ok();
            let effective_dry_run = dry_run || force_dry_run;

            let home_path = home::home_dir().ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "Cannot find home directory")
            })?;
            let config_path = home_path.join(config::CONFIG_FILE_NAME);
            if config_path.exists() {
                dotenvy::from_path(&config_path).ok();
            }

            let diff = git::get_staged_diff()?;
            if !effective_dry_run && diff.trim().is_empty() {
                println!("No staged changes to commit.");
                return Ok(());
            }

            // Get candidate count from environment variable
            let candidate_count = env::var(config::CANDIDATE_COUNT_ENV)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1);

            let provider = providers::ProviderFactory::create_provider();
            let response = provider.generate_commit_message(&diff, effective_dry_run, candidate_count)
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            
            if effective_dry_run {
                if candidate_count > 1 {
                    println!("Dry run: Would generate {} candidates", candidate_count);
                } else {
                    println!("{response}");
                }
                return Ok(());
            }
            
            // Parse the response into candidates
            let candidates = utils::parse_commit_message_candidates(&response, candidate_count);
            
            if candidates.is_empty() {
                println!("No commit message candidates generated.");
                return Ok(());
            }
            
            let selected_message = if candidates.len() == 1 {
                candidates[0].clone()
            } else {
                use dialoguer::Select;
                let selection = Select::new()
                    .with_prompt("Select a commit message")
                    .items(&candidates)
                    .default(0)
                    .interact()
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                
                candidates[selection].clone()
            };
            
            // Default behavior: automatically pipe to git commit --edit -F -
            commit::execute_git_commit_with_pipe(&selected_message)?;
        }
        Commands::Dev => {
            // Dev command is just an alias for generate --dry-run
            let generate_cmd = Commands::Generate { dry_run: true };
            return Box::pin(run(Cli {
                command: generate_cmd,
            }))
            .await;
        }
    }
    Ok(())
}