use anyhow::Result;
use colored::Colorize;
use crate::metadata::{AccountsMetadata, validate_account_name};
use crate::paths;
use crate::error::SwitchError;

/// Rename an existing account
pub fn rename(old: &str, new: &str) -> Result<()> {
    // Validate new name
    validate_account_name(new)?;

    // Load metadata
    let mut meta = AccountsMetadata::load()?;

    // Check if old account exists
    if !meta.account_exists(old) {
        return Err(SwitchError::AccountNotFound(old.to_string()).into());
    }

    // Check if new name already exists
    if meta.account_exists(new) {
        return Err(SwitchError::AccountExists(new.to_string()).into());
    }

    // Rename credentials file
    let old_path = paths::account_credentials_path(old)?;
    let new_path = paths::account_credentials_path(new)?;

    if old_path.exists() {
        std::fs::rename(&old_path, &new_path)?;
    }

    // Update metadata
    meta.rename_account(old, new)?;
    meta.save()?;

    println!();
    println!(
        "{} Renamed '{}' to '{}'",
        "✓".green(),
        old.dimmed(),
        new.cyan()
    );
    println!();

    Ok(())
}
