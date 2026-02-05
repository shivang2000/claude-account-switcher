# claude-switch

A fast, secure CLI to switch between Claude Pro accounts - **zero token usage**.

## Why?

The Claude Code plugin uses AI to execute file operations, consuming ~500 tokens per switch. This Rust CLI does it directly - instant, free, secure.

| | Plugin | CLI |
|---|--------|-----|
| **Speed** | 3-5 seconds | ~10ms |
| **Token cost** | ~500 tokens | 0 |
| **Works offline** | No | Yes |
| **Requires Claude** | Yes | No |

## Installation

### From source

```bash
git clone https://github.com/shivang2000/claude-account-switcher
cd claude-account-switcher/claude-switch
cargo install --path .
```

### From crates.io (coming soon)

```bash
cargo install claude-switch
```

## Usage

```bash
# Show current account
claude-switch current

# Save current credentials as an account
claude-switch add work

# List all saved accounts
claude-switch list

# Switch to a different account
claude-switch use personal

# Rename an account
claude-switch rename old-name new-name

# Remove an account
claude-switch remove old-account
```

**Important**: After switching, restart Claude Code for changes to take effect.

## Commands

| Command | Description |
|---------|-------------|
| `current` | Show current account info and token status |
| `add <name>` | Save current credentials as named account |
| `list` | List all saved accounts with status |
| `use <name>` | Switch to a different account |
| `remove <name>` | Delete a saved account |
| `rename <old> <new>` | Rename an existing account |

## Storage

Credentials are stored in `~/.claude/accounts/`:

```
~/.claude/
├── .credentials.json              # Active credentials
└── accounts/
    ├── .accounts.meta.json        # Metadata
    ├── .credentials.backup.json   # Auto-backup
    └── credentials/
        ├── work.json
        └── personal.json
```

All files use restrictive permissions (0600) for security.

## Compatibility

This CLI uses the same storage format as the Claude Code plugin. You can use both interchangeably.

## License

MIT
