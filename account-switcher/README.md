# Claude Account Switcher

A Claude Code plugin to switch between multiple Claude Pro accounts and extend your usage capacity.

## Why?

Instead of paying $100-200 for higher usage tiers, use multiple $20 Pro subscriptions and switch between them when you hit rate limits. This is completely legitimate - you're still paying for Claude Pro, just optimizing your usage across multiple subscriptions.

## Installation

### Option 1: Add Marketplace (Recommended)

1. Open Claude Code and run `/plugin`
2. Go to **Marketplaces** tab → **Add Marketplace**
3. Enter: `shivang2000/claude-account-switcher`
4. Go to **Discover** tab → Find **account-switcher** → **Install**
5. Restart Claude Code

### Option 2: Manual Installation

```bash
git clone https://github.com/shivang2000/claude-account-switcher.git
cp -r claude-account-switcher/account-switcher ~/.claude/plugins/
```

Then manually add to `~/.claude/plugins/installed_plugins.json` and `~/.claude/settings.json`.

### Option 3: Load for Development

```bash
git clone https://github.com/shivang2000/claude-account-switcher.git
claude --plugin-dir ./claude-account-switcher/account-switcher
```

## Commands

| Command | Description |
|---------|-------------|
| `/account-switcher:whoami` | Show current account info and token status |
| `/account-switcher:add-account <name>` | Save current credentials as named account |
| `/account-switcher:list-accounts` | Show all saved accounts with status |
| `/account-switcher:switch-account [name]` | Switch to different account (interactive if no name) |
| `/account-switcher:remove-account <name>` | Delete a saved account |
| `/account-switcher:rename-account <old> <new>` | Rename an existing account |

## Quick Start

### Step 1: Save Your Current Account

```
/account-switcher:add-account work
```

### Step 2: Add More Accounts

Log out of Claude (`claude logout`), log in with another Pro account, then:

```
/account-switcher:add-account personal
```

Repeat for as many accounts as you need (unlimited).

### Step 3: Switch Between Accounts

```
/account-switcher:switch-account work
```

Or use interactive mode:

```
/account-switcher:switch-account
```

**⚠️ Important**: Restart Claude Code after switching (`Ctrl+C` then `claude`).

### Step 4: Check Current Account

```
/account-switcher:whoami
```

## Features

- **Unlimited accounts** - Add as many Pro accounts as you need
- **Token tracking** - See expiration status for each account
- **Auto-backup** - Credentials backed up before every switch
- **Secure storage** - Files stored with 0600 permissions (owner read/write only)
- **Interactive prompts** - Easy account selection when no argument provided
- **Safety checks** - Can't delete active account, warns about expired tokens

## How It Works

The plugin stores your OAuth credentials in `~/.claude/accounts/`:

```
~/.claude/
├── .credentials.json              # Active credentials (used by Claude Code)
└── accounts/
    ├── .accounts.meta.json        # Account metadata (names, timestamps, etc.)
    ├── .credentials.backup.json   # Auto-backup before each switch
    └── credentials/
        ├── work.json              # Saved account credentials
        ├── personal.json
        └── client-a.json
```

**Switch process:**
1. Backs up current credentials
2. Validates target account exists and tokens aren't expired
3. Copies target credentials to `.credentials.json`
4. Updates metadata

**Why restart is required**: Claude Code loads OAuth tokens at startup and caches them in memory. There's no API to reload credentials without restarting.

## Token Status

| Status | Meaning |
|--------|---------|
| ✓ Valid | Token expires in 24+ hours |
| ⚠️ Warning | Token expires within 24 hours |
| ✗ EXPIRED | Token needs refresh (may require re-login) |

## Safety & Security

- **Local only** - Credentials never leave your machine
- **Restrictive permissions** - All files use 0600 (owner read/write only)
- **Auto-backup** - Backup created before every switch operation
- **Active account protection** - Can't accidentally delete currently active account
- **Token validation** - Warns before switching to expired tokens

## Troubleshooting

### Commands not appearing after install

Restart Claude Code. Plugins are loaded at startup.

### Token expired after switching

Claude Code auto-refreshes tokens on startup. If refresh fails, run `claude logout` then `claude login`.

### Lost credentials

Check the auto-backup at `~/.claude/accounts/.credentials.backup.json`

### "Unknown skill" error

Make sure the plugin is enabled in `/plugin` → **Installed** tab.

## Requirements

- Claude Code v1.0.33 or later
- One or more Claude Pro/Max subscriptions

## Use Cases

- **Individual developers** - Maximize coding time by rotating between Pro accounts
- **Small teams** - Share account pool for extended capacity
- **Heavy users** - Avoid hitting rate limits during intense coding sessions

## License

MIT - Feel free to use, modify, and distribute.

## Contributing

Issues and PRs welcome at [github.com/shivang2000/claude-account-switcher](https://github.com/shivang2000/claude-account-switcher)

## Credits

Built with Claude Code 🤖
