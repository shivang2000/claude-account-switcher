use clap::{Parser, Subcommand};
use colored::Colorize;

mod commands;
mod credentials;
mod error;
mod metadata;
mod paths;

#[derive(Parser)]
#[command(name = "claude-switch")]
#[command(author = "shivang2000")]
#[command(version)]
#[command(about = "Switch between Claude Pro accounts - zero token usage", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Save current credentials as a named account
    Add {
        /// Account name (letters, numbers, hyphens, underscores)
        name: String,

        /// Overwrite if account already exists
        #[arg(short, long)]
        force: bool,
    },

    /// List all saved accounts with status
    List,

    /// Switch to a different account (requires restart)
    Use {
        /// Account name to switch to
        name: String,
    },

    /// Remove a saved account
    Remove {
        /// Account name to remove
        name: String,
    },

    /// Rename an existing account
    Rename {
        /// Current account name
        old: String,

        /// New account name
        new: String,
    },

    /// Show current account info
    Current,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Add { name, force } => commands::add(&name, force),
        Commands::List => commands::list(),
        Commands::Use { name } => commands::use_account(&name),
        Commands::Remove { name } => commands::remove(&name),
        Commands::Rename { old, new } => commands::rename(&old, &new),
        Commands::Current => commands::current(),
    };

    if let Err(e) = result {
        eprintln!();
        eprintln!("{} {}", "Error:".red().bold(), e);
        eprintln!();
        std::process::exit(1);
    }
}
