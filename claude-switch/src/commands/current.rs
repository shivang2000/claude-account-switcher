use anyhow::Result;
use colored::Colorize;
use crate::credentials::{Credentials, TokenStatus};
use crate::metadata::AccountsMetadata;

/// Show current account info
pub fn current() -> Result<()> {
    let creds = Credentials::load_active()?;
    let meta = AccountsMetadata::load()?;

    let account_name = meta.current_account.as_deref().unwrap_or("Unknown");
    let subscription = creds.subscription_type();
    let status = TokenStatus::from_expires_at(creds.expires_at());

    println!();
    println!("{}", "Current Account".bold());
    println!("{}", "─".repeat(40));
    println!("  {} {}", "Account:".dimmed(), account_name.cyan());
    println!("  {} {}", "Subscription:".dimmed(), subscription);
    println!("  {} {}", "Token Status:".dimmed(), status.display());
    println!();

    if meta.current_account.is_none() {
        println!(
            "{}",
            "Tip: Use 'claude-switch add <name>' to save this account.".dimmed()
        );
        println!();
    }

    Ok(())
}
