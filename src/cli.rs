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