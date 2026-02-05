use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::error::{Result, SwitchError};
use crate::paths;

/// OAuth credentials for Claude AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeAiOauth {
    #[serde(rename = "accessToken")]
    pub access_token: String,

    #[serde(rename = "refreshToken")]
    pub refresh_token: String,

    #[serde(rename = "expiresAt")]
    pub expires_at: i64,

    pub scopes: Vec<String>,

    #[serde(rename = "subscriptionType")]
    pub subscription_type: String,

    #[serde(rename = "rateLimitTier")]
    pub rate_limit_tier: String,
}

/// Full credentials file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    #[serde(rename = "claudeAiOauth")]
    pub claude_ai_oauth: ClaudeAiOauth,

    #[serde(rename = "mcpOAuth", default, skip_serializing_if = "Option::is_none")]
    pub mcp_oauth: Option<serde_json::Value>,
}

impl Credentials {
    /// Read credentials from the active credentials file
    pub fn load_active() -> Result<Self> {
        let path = paths::credentials_path()?;
        Self::load_from(&path)
    }

    /// Read credentials from a specific path
    pub fn load_from(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(SwitchError::NoCredentials);
        }
        let contents = std::fs::read_to_string(path)?;
        let creds: Credentials = serde_json::from_str(&contents)?;
        Ok(creds)
    }

    /// Save credentials to a specific path with secure permissions
    pub fn save_to(&self, path: &Path) -> Result<()> {
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(path, contents)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
        }

        Ok(())
    }

    /// Save as the active credentials
    pub fn save_active(&self) -> Result<()> {
        let path = paths::credentials_path()?;
        self.save_to(&path)
    }

    /// Get subscription type
    pub fn subscription_type(&self) -> &str {
        &self.claude_ai_oauth.subscription_type
    }

    /// Get token expiration timestamp
    pub fn expires_at(&self) -> i64 {
        self.claude_ai_oauth.expires_at
    }
}

/// Token status based on expiration
#[derive(Debug, Clone, PartialEq)]
pub enum TokenStatus {
    Valid { days: i64 },
    Warning { hours: i64 },
    Expired,
}

impl TokenStatus {
    /// Calculate status from expiration timestamp (milliseconds)
    pub fn from_expires_at(expires_at: i64) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        let hours_remaining = (expires_at - now) / (1000 * 60 * 60);

        if hours_remaining < 0 {
            TokenStatus::Expired
        } else if hours_remaining < 24 {
            TokenStatus::Warning { hours: hours_remaining }
        } else {
            TokenStatus::Valid { days: hours_remaining / 24 }
        }
    }

    /// Get a colored display string
    pub fn display(&self) -> String {
        use colored::Colorize;
        match self {
            TokenStatus::Valid { days } => format!("{} Valid ({} days)", "✓".green(), days),
            TokenStatus::Warning { hours } => format!("{} {} hours", "⚠".yellow(), hours),
            TokenStatus::Expired => format!("{} Expired", "✗".red()),
        }
    }
}
