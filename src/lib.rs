mod api;
mod cli;
mod config;
mod convention;
mod git;

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
            let config_path = home_path.join(".committorc");

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
            let force_dry_run = env::var("COMMITTO_DEV").is_ok();
            let effective_dry_run = dry_run || force_dry_run;

            let home_path = home::home_dir().ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "Cannot find home directory")
            })?;
            let config_path = home_path.join(".committorc");
            if config_path.exists() {
                dotenvy::from_path(&config_path).ok();
            }

            let diff = git::get_staged_diff()?;
            if !effective_dry_run && diff.trim().is_empty() {
                println!("No staged changes to commit.");
                return Ok(());
            }

            let commit_message = api::generate_commit_message(&diff, effective_dry_run)
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            println!("{commit_message}");
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