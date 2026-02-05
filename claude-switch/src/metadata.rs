use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::{Result, SwitchError};
use crate::paths;

/// Information about a single saved account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    #[serde(rename = "addedAt")]
    pub added_at: i64,

    #[serde(rename = "lastUsedAt")]
    pub last_used_at: i64,

    #[serde(rename = "subscriptionType")]
    pub subscription_type: String,

    #[serde(rename = "tokenExpiresAt")]
    pub token_expires_at: i64,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Metadata for all saved accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountsMetadata {
    pub version: u32,

    #[serde(rename = "currentAccount", skip_serializing_if = "Option::is_none")]
    pub current_account: Option<String>,

    #[serde(rename = "lastBackupAt", skip_serializing_if = "Option::is_none")]
    pub last_backup_at: Option<i64>,

    pub accounts: HashMap<String, AccountInfo>,
}

impl Default for AccountsMetadata {
    fn default() -> Self {
        Self {
            version: 1,
            current_account: None,
            last_backup_at: None,
            accounts: HashMap::new(),
        }
    }
}

impl AccountsMetadata {
    /// Load metadata from file, or return default if not exists
    pub fn load() -> Result<Self> {
        let path = paths::metadata_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = std::fs::read_to_string(&path)?;
        let meta: AccountsMetadata = serde_json::from_str(&contents)?;
        Ok(meta)
    }

    /// Save metadata to file
    pub fn save(&self) -> Result<()> {
        paths::ensure_accounts_dir()?;
        let path = paths::metadata_path()?;
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, contents)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
        }

        Ok(())
    }

    /// Check if an account exists
    pub fn account_exists(&self, name: &str) -> bool {
        self.accounts.contains_key(name)
    }

    /// Get account info
    #[allow(dead_code)]
    pub fn get_account(&self, name: &str) -> Option<&AccountInfo> {
        self.accounts.get(name)
    }

    /// Add or update an account
    pub fn add_account(&mut self, name: String, info: AccountInfo) {
        self.accounts.insert(name, info);
    }

    /// Remove an account
    pub fn remove_account(&mut self, name: &str) -> Option<AccountInfo> {
        self.accounts.remove(name)
    }

    /// Rename an account
    pub fn rename_account(&mut self, old: &str, new: &str) -> Result<()> {
        if let Some(info) = self.accounts.remove(old) {
            self.accounts.insert(new.to_string(), info);
            if self.current_account.as_deref() == Some(old) {
                self.current_account = Some(new.to_string());
            }
            Ok(())
        } else {
            Err(SwitchError::AccountNotFound(old.to_string()))
        }
    }

    /// Get list of account names
    #[allow(dead_code)]
    pub fn account_names(&self) -> Vec<&str> {
        self.accounts.keys().map(|s| s.as_str()).collect()
    }

    /// Check if there are any saved accounts
    pub fn has_accounts(&self) -> bool {
        !self.accounts.is_empty()
    }
}

/// Validate account name
pub fn validate_account_name(name: &str) -> Result<()> {
    let is_valid = name.len() >= 2
        && name.len() <= 30
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_');

    if is_valid {
        Ok(())
    } else {
        Err(SwitchError::InvalidAccountName(name.to_string()))
    }
}
