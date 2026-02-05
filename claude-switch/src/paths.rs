use std::path::PathBuf;
use crate::error::{Result, SwitchError};

/// Get the Claude configuration directory (~/.claude)
pub fn claude_dir() -> Result<PathBuf> {
    dirs::home_dir()
        .map(|h| h.join(".claude"))
        .ok_or(SwitchError::NoHomeDir)
}

/// Get the path to the active credentials file
pub fn credentials_path() -> Result<PathBuf> {
    Ok(claude_dir()?.join(".credentials.json"))
}

/// Get the accounts storage directory
pub fn accounts_dir() -> Result<PathBuf> {
    Ok(claude_dir()?.join("accounts"))
}

/// Get the credentials subdirectory
pub fn credentials_dir() -> Result<PathBuf> {
    Ok(accounts_dir()?.join("credentials"))
}

/// Get the path to account metadata file
pub fn metadata_path() -> Result<PathBuf> {
    Ok(accounts_dir()?.join(".accounts.meta.json"))
}

/// Get the path to the backup file
pub fn backup_path() -> Result<PathBuf> {
    Ok(accounts_dir()?.join(".credentials.backup.json"))
}

/// Get the path to a specific account's credentials
pub fn account_credentials_path(name: &str) -> Result<PathBuf> {
    Ok(credentials_dir()?.join(format!("{}.json", name)))
}

/// Ensure accounts directory structure exists
pub fn ensure_accounts_dir() -> Result<()> {
    let creds_dir = credentials_dir()?;
    if !creds_dir.exists() {
        std::fs::create_dir_all(&creds_dir)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            // Set directory permissions to 0700
            std::fs::set_permissions(accounts_dir()?, std::fs::Permissions::from_mode(0o700))?;
            std::fs::set_permissions(&creds_dir, std::fs::Permissions::from_mode(0o700))?;
        }
    }
    Ok(())
}
