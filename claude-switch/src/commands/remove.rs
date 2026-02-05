use anyhow::Result;
use colored::Colorize;
use crate::metadata::AccountsMetadata;
use crate::paths;
use crate::error::SwitchError;

/// Remove a saved account
pub fn remove(name: &str) -> Result<()> {
    // Load metadata
    let mut meta = AccountsMetadata::load()?;

    // Check if account exists
    if !meta.account_exists(name) {
        return Err(SwitchError::AccountNotFound(name.to_string()).into());
    }

    // Check if trying to remove active account
    if meta.current_account.as_deref() == Some(name) {
        return Err(SwitchError::CannotRemoveActive(name.to_string()).into());
    }

    // Remove credentials file
    let account_path = paths::account_credentials_path(name)?;
    if account_path.exists() {
        std::fs::remove_file(&account_path)?;
    }

    // Remove from metadata
    meta.remove_account(name);
    meta.save()?;

    println!();
    println!("{} Account '{}' removed", "✓".green(), name.cyan());
    println!();

    Ok(())
}
