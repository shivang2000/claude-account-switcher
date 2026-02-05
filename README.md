# Claude Account Switcher

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Switch between multiple Claude Pro accounts and extend your usage capacity.

## Two Options

| | Claude Code Plugin | Rust CLI |
|---|---|---|
| **Install** | `/plugin` → Add marketplace | `cargo install --path claude-switch` |
| **Speed** | 3-5 seconds | ~10ms |
| **Token cost** | ~500 tokens | **0** |
| **Works offline** | No | Yes |

## Option 1: Claude Code Plugin

### Install

1. Run `/plugin` in Claude Code
2. Go to **Marketplaces** → **Add Marketplace**
3. Enter: `shivang2000/claude-account-switcher`
4. Go to **Discover** → Install **account-switcher**
5. Restart Claude Code

### Usage

```bash
/account-switcher:add-account work
/account-switcher:list-accounts
/account-switcher:switch-account personal
/account-switcher:whoami
```

## Option 2: Rust CLI (Recommended)

Zero token usage, instant execution.

### Install

```bash
git clone https://github.com/shivang2000/claude-account-switcher
cd claude-account-switcher/claude-switch
cargo install --path .
```

### Usage

```bash
claude-switch add work
claude-switch list
claude-switch use personal
claude-switch current
```

## Why?

Instead of paying $100-200 for higher usage tiers, use multiple $20 Pro subscriptions and rotate between them when you hit rate limits.

**This is 100% legitimate** - you're still paying for Claude Pro subscriptions, just optimizing usage across multiple accounts.

## Features

- ✅ Unlimited accounts
- ✅ Token expiration tracking
- ✅ Auto-backup before switch
- ✅ Secure storage (0600 permissions)
- ✅ Interactive account picker

## Documentation

- [Plugin Documentation](./account-switcher/README.md)
- [CLI Documentation](./claude-switch/README.md)

## License

MIT
