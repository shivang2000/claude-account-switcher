# Claude Account Switcher - Rust CLI

## Overview

A standalone Rust CLI tool to switch between multiple Claude Pro accounts without using any Claude tokens. This complements the existing Claude Code plugin with a faster, more secure, token-free alternative.

## Why Rust CLI?

| Plugin | Rust CLI |
|--------|----------|
| Uses ~500 tokens per operation | Zero token usage |
| 3-5 seconds per command | ~10ms execution |
| Requires Claude Code running | Works standalone |
| Credentials in plain JSON | Optional encryption |
| Cross-platform via Claude | Native binary |

## Features

- **Zero token usage** - Direct file operations, no AI involved
- **Fast** - Native binary, instant execution
- **Secure** - Optional keychain/keyring integration
- **Offline** - Works without internet (except login)
- **Same storage format** - Compatible with plugin's data

## CLI Commands

```bash
# Account management
claude-switch add <name>        # Save current credentials as named account
claude-switch list              # List all saved accounts with status
claude-switch use <name>        # Switch to account (then restart Claude)
claude-switch remove <name>     # Delete saved account
claude-switch rename <old> <new> # Rename account
claude-switch current           # Show current account (like whoami)

# Utility
claude-switch backup            # Manual backup of all credentials
claude-switch doctor            # Check for issues (expired tokens, etc.)
```

## Project Structure

```
claude-switch/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry point (clap)
│   ├── lib.rs               # Library exports
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── add.rs           # Add account
│   │   ├── list.rs          # List accounts
│   │   ├── use_account.rs   # Switch account
│   │   ├── remove.rs        # Remove account
│   │   ├── rename.rs        # Rename account
│   │   └── current.rs       # Show current
│   ├── credentials.rs       # Read/write credentials
│   ├── metadata.rs          # Account metadata handling
│   ├── paths.rs             # Path resolution (~/.claude/...)
│   └── error.rs             # Error types
├── tests/
│   └── integration_tests.rs
└── README.md
```

## Dependencies

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }    # CLI parsing
serde = { version = "1", features = ["derive"] }   # JSON serialization
serde_json = "1"                                    # JSON parsing
chrono = "0.4"                                      # Timestamp handling
dirs = "5"                                          # Home directory
colored = "2"                                       # Terminal colors
anyhow = "1"                                        # Error handling
thiserror = "1"                                     # Custom errors

[target.'cfg(target_os = "macos")'.dependencies]
keyring = "2"                                       # macOS Keychain (optional)
```

## Data Structures

### Credentials (from ~/.claude/.credentials.json)
```rust
#[derive(Serialize, Deserialize)]
struct Credentials {
    #[serde(rename = "claudeAiOauth")]
    claude_ai_oauth: ClaudeAiOauth,
    #[serde(rename = "mcpOAuth")]
    mcp_oauth: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
struct ClaudeAiOauth {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    #[serde(rename = "expiresAt")]
    expires_at: i64,
    scopes: Vec<String>,
    #[serde(rename = "subscriptionType")]
    subscription_type: String,
    #[serde(rename = "rateLimitTier")]
    rate_limit_tier: String,
}
```

### Metadata (from ~/.claude/accounts/.accounts.meta.json)
```rust
#[derive(Serialize, Deserialize)]
struct AccountsMetadata {
    version: u32,
    #[serde(rename = "currentAccount")]
    current_account: Option<String>,
    #[serde(rename = "lastBackupAt")]
    last_backup_at: Option<i64>,
    accounts: HashMap<String, AccountInfo>,
}

#[derive(Serialize, Deserialize)]
struct AccountInfo {
    #[serde(rename = "addedAt")]
    added_at: i64,
    #[serde(rename = "lastUsedAt")]
    last_used_at: i64,
    #[serde(rename = "subscriptionType")]
    subscription_type: String,
    #[serde(rename = "tokenExpiresAt")]
    token_expires_at: i64,
    notes: Option<String>,
}
```

## Implementation Order

1. **Setup** - Cargo.toml, project structure
2. **paths.rs** - Path resolution for ~/.claude directories
3. **credentials.rs** - Read/write credential files
4. **metadata.rs** - Account metadata handling
5. **commands/current.rs** - Show current account (simplest)
6. **commands/list.rs** - List all accounts
7. **commands/add.rs** - Add new account
8. **commands/use_account.rs** - Switch accounts
9. **commands/remove.rs** - Remove account
10. **commands/rename.rs** - Rename account
11. **main.rs** - CLI with clap
12. **Tests** - Integration tests
13. **README.md** - Documentation

## Key Implementation Details

### Path Resolution (paths.rs)
```rust
pub fn claude_dir() -> PathBuf {
    dirs::home_dir().unwrap().join(".claude")
}

pub fn credentials_path() -> PathBuf {
    claude_dir().join(".credentials.json")
}

pub fn accounts_dir() -> PathBuf {
    claude_dir().join("accounts")
}

pub fn metadata_path() -> PathBuf {
    accounts_dir().join(".accounts.meta.json")
}

pub fn account_credentials_path(name: &str) -> PathBuf {
    accounts_dir().join("credentials").join(format!("{}.json", name))
}
```

### Permission Handling
```rust
#[cfg(unix)]
fn set_secure_permissions(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let perms = std::fs::Permissions::from_mode(0o600);
    std::fs::set_permissions(path, perms)?;
    Ok(())
}
```

### Token Status
```rust
fn token_status(expires_at: i64) -> TokenStatus {
    let now = chrono::Utc::now().timestamp_millis();
    let hours_remaining = (expires_at - now) / (1000 * 60 * 60);

    if hours_remaining < 0 {
        TokenStatus::Expired
    } else if hours_remaining < 24 {
        TokenStatus::Warning(hours_remaining)
    } else {
        TokenStatus::Valid(hours_remaining / 24)
    }
}
```

## CLI Design (main.rs)

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "claude-switch")]
#[command(about = "Switch between Claude Pro accounts")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Save current credentials as named account
    Add { name: String },
    /// List all saved accounts
    List,
    /// Switch to a different account
    Use { name: String },
    /// Remove a saved account
    Remove { name: String },
    /// Rename an account
    Rename { old: String, new: String },
    /// Show current account info
    Current,
    /// Check for issues
    Doctor,
}
```

## Output Examples

```bash
$ claude-switch current
Current Account: work
Subscription: pro
Token Status: ✓ Valid (5 days remaining)

$ claude-switch list
┌────────┬──────┬──────────────────┬─────────────┐
│ Status │ Name │ Subscription     │ Token       │
├────────┼──────┼──────────────────┼─────────────┤
│ ● work │ pro  │ ✓ Valid (5 days) │             │
│   personal     │ pro  │ ⚠ 12 hours      │             │
│   testing      │ pro  │ ✗ Expired       │             │
└────────┴──────┴──────────────────┴─────────────┘

$ claude-switch use personal
✓ Backed up current credentials
✓ Switched to 'personal'

⚠ Restart Claude Code to apply changes
```

## Installation

```bash
# From crates.io (after publishing)
cargo install claude-switch

# From source
git clone https://github.com/shivang2000/claude-account-switcher
cd claude-switch
cargo install --path .
```

## Verification

1. `cargo build` - Compiles without errors
2. `cargo test` - All tests pass
3. `claude-switch current` - Shows current account
4. `claude-switch add test` - Saves account
5. `claude-switch list` - Shows saved accounts
6. `claude-switch use test` - Switches (then restart Claude)
7. `claude-switch remove test` - Removes account

## Repository Structure Update

```
claude-account-switcher/
├── README.md                    # Main readme
├── .claude-plugin/
│   └── marketplace.json
├── account-switcher/            # Claude Code plugin
│   ├── README.md
│   ├── .claude-plugin/
│   └── commands/
└── claude-switch/               # Rust CLI (NEW)
    ├── Cargo.toml
    ├── README.md
    └── src/
```

## Files to Create

| File | Purpose |
|------|---------|
| `claude-switch/Cargo.toml` | Project dependencies |
| `claude-switch/src/main.rs` | CLI entry point |
| `claude-switch/src/lib.rs` | Library exports |
| `claude-switch/src/paths.rs` | Path resolution |
| `claude-switch/src/credentials.rs` | Credential handling |
| `claude-switch/src/metadata.rs` | Metadata handling |
| `claude-switch/src/error.rs` | Error types |
| `claude-switch/src/commands/*.rs` | Individual commands |
| `claude-switch/README.md` | CLI documentation |
