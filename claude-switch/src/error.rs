use thiserror::Error;

#[derive(Error, Debug)]
pub enum SwitchError {
    #[error("No credentials found. Please login to Claude Code first.")]
    NoCredentials,

    #[error("Account '{0}' not found")]
    AccountNotFound(String),

    #[error("Account '{0}' already exists. Use --force to overwrite.")]
    AccountExists(String),

    #[error("Cannot remove active account '{0}'. Switch to another account first.")]
    CannotRemoveActive(String),

    #[error("Invalid account name '{0}'. Use only letters, numbers, hyphens, and underscores (2-30 chars).")]
    InvalidAccountName(String),

    #[error("No accounts saved yet. Use 'claude-switch add <name>' to save your first account.")]
    NoAccountsSaved,

    #[error("Home directory not found")]
    NoHomeDir,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, SwitchError>;
