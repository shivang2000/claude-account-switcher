use anyhow::Result;
use colored::Colorize;
use crate::credentials::Credentials;
use crate::metadata::{AccountInfo, AccountsMetadata, validate_account_name};
use crate::paths;
use crate::error::SwitchError;

/// Add/save current credentials as a named account
pub fn add(name: &str, force: bool) -> Result<()> {
    // Validate name
    validate_account_name(name)?;

    // Load current credentials
    let creds = Credentials::load_active()?;

    // Load metadata
    let mut meta = AccountsMetadata::load()?;

    // Check if account already exists
    if meta.account_exists(name) && !force {
        return Err(SwitchError::AccountExists(name.to_string()).into());
    }

    // Ensure directory structure exists
    paths::ensure_accounts_dir()?;

    // Save credentials to account file
    let account_path = paths::account_credentials_path(name)?;
    creds.save_to(&account_path)?;

    // Update metadata
    let now = chrono::Utc::now().timestamp_millis();
    let info = AccountInfo {
        added_at: now,
        last_used_at: now,
        subscription_type: creds.subscription_type().to_string(),
        token_expires_at: creds.expires_at(),
        notes: None,
    };

    meta.add_account(name.to_string(), info);
    meta.current_account = Some(name.to_string());
    meta.save()?;

    println!();
    println!("{} Account '{}' saved successfully!", "✓".green(), name.cyan());
    println!();
    println!("  {} {}", "Subscription:".dimmed(), creds.subscription_type());
    println!(
        "  {} {}",
        "Stored at:".dimmed(),
        account_path.display().to_string().dimmed()
    );
    println!();

    Ok(())
}
