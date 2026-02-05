use anyhow::Result;
use colored::Colorize;
use crate::credentials::{Credentials, TokenStatus};
use crate::metadata::AccountsMetadata;
use crate::paths;
use crate::error::SwitchError;

/// Switch to a different account
pub fn use_account(name: &str) -> Result<()> {
    // Load metadata
    let mut meta = AccountsMetadata::load()?;

    // Check if account exists
    if !meta.account_exists(name) {
        return Err(SwitchError::AccountNotFound(name.to_string()).into());
    }

    // Check if already using this account
    if meta.current_account.as_deref() == Some(name) {
        println!();
        println!("{} Already using account '{}'", "ℹ".blue(), name.cyan());
        println!();
        return Ok(());
    }

    // Load target credentials
    let account_path = paths::account_credentials_path(name)?;
    let target_creds = Credentials::load_from(&account_path)?;

    // Check token status and warn if expired
    let status = TokenStatus::from_expires_at(target_creds.expires_at());
    if let TokenStatus::Expired = status {
        println!();
        println!(
            "{} Token for '{}' is expired. You may need to re-login after switching.",
            "⚠".yellow(),
            name
        );
        println!();
    }

    // Backup current credentials
    let current_creds = Credentials::load_active()?;
    let backup_path = paths::backup_path()?;
    current_creds.save_to(&backup_path)?;

    // Update backup timestamp in metadata
    meta.last_backup_at = Some(chrono::Utc::now().timestamp_millis());

    // Copy target credentials to active
    target_creds.save_active()?;

    // Update metadata
    let now = chrono::Utc::now().timestamp_millis();
    if let Some(info) = meta.accounts.get_mut(name) {
        info.last_used_at = now;
    }
    meta.current_account = Some(name.to_string());
    meta.save()?;

    println!();
    println!("{} Backed up current credentials", "✓".green());
    println!("{} Switched to '{}'", "✓".green(), name.cyan());
    println!();
    println!(
        "{}",
        "⚠  Restart Claude Code to apply changes".yellow().bold()
    );
    println!();

    Ok(())
}
