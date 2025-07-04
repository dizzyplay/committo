use clap::{Parser, Subcommand};

/// Commit message generator powered by LLM
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Set a config value
    Set {
        /// Config key
        key: String,
        /// Config value
        value: String,
    },
    /// Show current config
    Show,
    /// Generate a commit message
    Generate {
        /// Run in dry-run mode to see the generated prompt without calling the API
        #[arg(long)]
        dry_run: bool,
    },
    /// Quick dry-run (alias for generate --dry-run)
    Dev,
}