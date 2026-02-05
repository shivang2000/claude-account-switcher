use anyhow::Result;
use colored::Colorize;
use crate::credentials::TokenStatus;
use crate::metadata::AccountsMetadata;
use crate::error::SwitchError;

/// List all saved accounts
pub fn list() -> Result<()> {
    let meta = AccountsMetadata::load()?;

    if !meta.has_accounts() {
        return Err(SwitchError::NoAccountsSaved.into());
    }

    let current = meta.current_account.as_deref();

    println!();
    println!("{}", "Saved Accounts".bold());
    println!("{}", "─".repeat(60));
    println!(
        "  {:<2} {:<15} {:<10} {}",
        "", "NAME".dimmed(), "TYPE".dimmed(), "TOKEN STATUS".dimmed()
    );
    println!("{}", "─".repeat(60));

    // Sort accounts by name
    let mut accounts: Vec<_> = meta.accounts.iter().collect();
    accounts.sort_by(|a, b| a.0.cmp(b.0));

    for (name, info) in accounts {
        let is_current = current == Some(name.as_str());
        let marker = if is_current { "●".green().to_string() } else { " ".to_string() };
        let status = TokenStatus::from_expires_at(info.token_expires_at);

        let name_display = if is_current {
            name.cyan().bold().to_string()
        } else {
            name.to_string()
        };

        println!(
            "  {:<2} {:<15} {:<10} {}",
            marker,
            name_display,
            info.subscription_type,
            status.display()
        );
    }

    println!("{}", "─".repeat(60));
    println!();
    println!(
        "{}",
        format!("  {} = active account", "●".green()).dimmed()
    );
    println!();

    Ok(())
}
