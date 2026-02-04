# Claude Account Switcher Tool

## Overview
A CLI command/plugin for Claude Code that enables switching between multiple Claude Pro accounts to extend usage capacity.

## How It Works

### Credential Storage Structure
```
~/.claude/
├── .credentials.json          # Active credentials (used by Claude Code)
└── accounts/                  # New directory for stored accounts
    ├── account-1.json         # Backup of account 1 credentials
    ├── account-2.json         # Backup of account 2 credentials
    └── accounts.meta.json     # Account metadata (names, last used, etc.)
```

### Core Commands

| Command | Description |
|---------|-------------|
| `/switch-account` | Interactive account picker |
| `/switch-account <name>` | Switch to specific account |
| `/add-account <name>` | Save current credentials as named account |
| `/list-accounts` | Show all saved accounts with status |
| `/remove-account <name>` | Delete saved account |

## Implementation: Claude Code Plugin

Create a native plugin with slash commands for seamless account management.

### Plugin Structure
```
~/.claude/plugins/account-switcher/
├── plugin.json                    # Plugin manifest
├── commands/
│   ├── switch-account.md          # /switch-account command
│   ├── add-account.md             # /add-account command
│   ├── list-accounts.md           # /list-accounts command
│   └── remove-account.md          # /remove-account command
└── README.md                      # Usage docs
```

## Technical Details

### Credential File Format
```json
{
  "claudeAiOauth": {
    "accessToken": "sk-ant-oat01-...",
    "refreshToken": "sk-ant-ort01-...",
    "expiresAt": 1770252787339,
    "scopes": ["user:inference", "user:profile", ...],
    "subscriptionType": "pro",
    "rateLimitTier": "default_claude_ai"
  },
  "mcpOAuth": { ... }
}
```

### Switch Logic
1. Backup current `.credentials.json` to `accounts/<current>.json`
2. Copy target `accounts/<target>.json` to `.credentials.json`
3. Notify user to restart Claude Code
4. Update `accounts.meta.json` with last-used timestamp

### Session Handling
- Claude Code must be restarted after switching (OAuth tokens are loaded at startup)
- Token refresh happens automatically if tokens are still valid
- If tokens expired, user will be prompted to re-authenticate

## Files to Create

### 1. Plugin Manifest (`plugin.json`)
```json
{
  "name": "account-switcher",
  "version": "1.0.0",
  "description": "Switch between multiple Claude accounts"
}
```

### 2. Commands

**`/switch-account`** - Interactive picker or direct switch
- Reads available accounts from `~/.claude/accounts/`
- Shows interactive picker if no argument
- Backs up current credentials, swaps in target
- Instructs user to restart Claude Code

**`/add-account <name>`** - Save current session
- Copies current `.credentials.json` to `accounts/<name>.json`
- Creates accounts directory if needed
- Validates name (no special chars)

**`/list-accounts`** - Show saved accounts
- Lists all `.json` files in accounts directory
- Shows subscription type, last modified date

**`/remove-account <name>`** - Delete saved account
- Removes specified account file
- Confirms before deletion

## User Flow

### First-time Setup
1. Login to Account 1 normally
2. Run `/add-account work` → saves current creds as "work"
3. Logout, login to Account 2
4. Run `/add-account personal` → saves current creds as "personal"
5. Now can switch freely with `/switch-account`

### Daily Usage
```
$ claude
> /switch-account

Select account:
  ● work (last used: 2 hours ago)
  ○ personal (last used: yesterday)

Switched to 'work'. Please restart Claude Code.
```

## Verification
1. Create plugin structure
2. Add first account, verify it's saved
3. Login to second account, add it
4. Switch between accounts, verify credentials swap correctly
5. Confirm Claude Code works with both accounts after restart

## Considerations
- Token expiration: Stored tokens may expire if unused for extended periods
- Re-authentication: User may need to re-login if refresh token expires
- MCP credentials: Are account-specific, will switch along with main credentials
