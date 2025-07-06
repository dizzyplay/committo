use clap::{Parser, Subcommand};

const VERSION: &str = match option_env!("BUILD_VERSION") {
    Some(v) => v,
    None => env!("CARGO_PKG_VERSION"),
};

/// Commit message generator powered by LLM
#[derive(Parser, Debug)]
#[command(version = VERSION, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    /// Run in dry-run mode to see the generated prompt without calling the API
    #[arg(long, global = true)]
    pub dry_run: bool,
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
    Generate,
}