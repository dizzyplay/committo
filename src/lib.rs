pub mod api;
mod cli;
mod commit;
pub mod config;
mod convention;
mod git;
pub mod providers;
mod utils;

pub use cli::{Cli, Commands};

use std::io;

/// Main application runner
pub async fn run(cli: Cli) -> io::Result<()> {
    // Get home directory and config path
    let home_path = home::home_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Cannot find home directory")
    })?;
    let config_path = home_path.join(config::CONFIG_FILE_NAME);
    match cli.command {
        Some(Commands::Set { key, value }) => {
            // Set command doesn't need interactive setup - just use static method
            config::Config::handle_set_command(&key, &value, &config_path)?;
        }
        Some(Commands::Show) => {
            // Create config instance - this will handle setup if needed
            let (config, _) = config::Config::new(&config_path)?;
            config.show()?;
        }
        Some(Commands::Generate) | None => {
            // Default to generate when no subcommand is provided
            // Create config instance - this will handle setup if needed
            let (config, _) = config::Config::new(&config_path)?;
            let provider = providers::ProviderFactory::create_provider(config);
            
            // Get effective dry run mode from global CLI flag
            let effective_dry_run = cli.dry_run;

            let diff = git::get_staged_diff()?;
            if !effective_dry_run && diff.trim().is_empty() {
                println!("No staged changes to commit.");
                return Ok(());
            }

            let mut response = provider.generate_commit_message(&diff, effective_dry_run)
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            
            if effective_dry_run {
                let candidate_count = provider.get_candidate_count();
                if candidate_count > 1 {
                    println!("Dry run: Would generate {} candidates", candidate_count);
                } else {
                    println!("{response}");
                }
                return Ok(());
            }
            
            // Parse the response into candidates and handle selection with retry
            let candidate_count = provider.get_candidate_count();
            let selected_message = loop {
                let candidates = utils::parse_commit_message_candidates(&response, candidate_count);
                
                if candidates.is_empty() {
                    println!("No commit message candidates generated.");
                    return Ok(());
                }
                
                if candidates.len() == 1 {
                    // Single candidate - ask if user wants to retry or use it
                    use inquire::Select;
                    let options = vec!["🔄 Retry (generate new commit message)", &candidates[0]];
                    let selection = Select::new("Select an option:", options.clone())
                        .with_starting_cursor(1) // Default to the generated message
                        .prompt()
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                    let selection_index = options.iter().position(|&x| x == selection).unwrap();
                    
                    if selection_index == 0 {
                        // Retry - generate new message
                        println!("🔄 Generating new commit message...");
                        let new_response = provider.generate_commit_message(&diff, false)
                            .await
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                        response = new_response;
                        continue;
                    } else {
                        break candidates[0].clone();
                    }
                } else {
                    // Multiple candidates - add retry option at the top
                    use inquire::Select;
                    let mut options = vec!["🔄 Retry (generate new messages)".to_string()];
                    options.extend(candidates.iter().cloned());
                    
                    let selection = Select::new("Select a commit message:", options.clone())
                        .with_starting_cursor(1) // Default to first generated message
                        .prompt()
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                    let selection_index = options.iter().position(|x| x == &selection).unwrap();
                    
                    if selection_index == 0 {
                        // Retry - generate new messages
                        println!("🔄 Generating new commit messages...");
                        let new_response = provider.generate_commit_message(&diff, false)
                            .await
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                        response = new_response;
                        continue;
                    } else {
                        break candidates[selection_index - 1].clone();
                    }
                }
            };
            
            // Default behavior: automatically pipe to git commit --edit -F -
            commit::execute_git_commit_with_pipe(&selected_message)?;
        }
    }
    Ok(())
}