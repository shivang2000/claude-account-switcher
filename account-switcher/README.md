# Claude Account Switcher

A Claude Code plugin to switch between multiple Claude Pro accounts and extend your usage capacity.

## Why?

Instead of paying $100-200 for higher usage tiers, use multiple $20 Pro subscriptions and switch between them when you hit rate limits.

## Installation

### Option 1: Install from GitHub (Recommended)

Add this marketplace to your Claude Code settings, then install the plugin:

```bash
# Add the marketplace
claude config add pluginMarketplaces "https://raw.githubusercontent.com/shivang2000/claude-account-switcher/main/marketplace.json"

# Install the plugin
claude plugin install account-switcher
```

Or use the interactive plugin manager:
```
/plugin
# Navigate to "Discover" tab
# Search for "account-switcher"
# Click Install
```

### Option 2: Manual Installation

Clone and copy to your plugins directory:

```bash
git clone https://github.com/shivang2000/claude-account-switcher.git
cp -r claude-account-switcher/account-switcher ~/.claude/plugins/
```

### Option 3: Load for Development

Test without installing:

```bash
claude --plugin-dir ./account-switcher
```

## Commands

| Command | Description |
|---------|-------------|
| `/account-switcher:whoami` | Show current account info and token status |
| `/account-switcher:add-account <name>` | Save current credentials as named account |
| `/account-switcher:list-accounts` | Show all saved accounts with status |
| `/account-switcher:switch-account [name]` | Switch to different account |
| `/account-switcher:remove-account <name>` | Delete a saved account |
| `/account-switcher:rename-account <old> <new>` | Rename an existing account |

## Quick Start

### 1. Save Your First Account

```
/account-switcher:add-account work
```

### 2. Add More Accounts

Log out, log in with another account, then:

```
/account-switcher:add-account personal
```

Repeat for as many accounts as you need.

### 3. Switch Between Accounts

```
/account-switcher:switch-account
```

Select from the interactive picker, then **restart Claude Code**.

## Features

- **Unlimited accounts** - Add as many Pro accounts as you need
- **Token tracking** - See expiration status for each account
- **Auto-backup** - Credentials backed up before every switch
- **Secure storage** - Files stored with 0600 permissions
- **Interactive prompts** - Easy account selection

## How It Works

The plugin stores your OAuth credentials in `~/.claude/accounts/`:

```
~/.claude/
├── .credentials.json              # Active credentials (used by Claude)
└── accounts/
    ├── .accounts.meta.json        # Account metadata
    ├── .credentials.backup.json   # Auto-backup
    └── credentials/
        ├── work.json
        └── personal.json
```

When you switch accounts, it:
1. Backs up current credentials
2. Copies target account credentials to `.credentials.json`
3. Updates metadata

**Important**: Claude Code must be restarted after switching (OAuth tokens are loaded at startup).

## Token Status

| Status | Meaning |
|--------|---------|
| Valid | Token expires in 24+ hours |
| Warning | Token expires within 24 hours |
| EXPIRED | Token needs refresh (may require re-login) |

## Safety

- Credentials never leave your machine
- All files use restrictive permissions (0600)
- Backup created before every switch
- Can't accidentally delete active account

## Troubleshooting

### Commands not appearing

Restart Claude Code after installation.

### Token expired

Claude Code auto-refreshes on startup. If that fails, log in again.

### Lost credentials

Check `~/.claude/accounts/.credentials.backup.json`

## Requirements

- Claude Code v1.0.33 or later
- Multiple Claude Pro subscriptions

## License

MIT

## Contributing

Issues and PRs welcome!
