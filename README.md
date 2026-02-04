# Claude Account Switcher

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Claude Code plugin to switch between multiple Claude Pro accounts and extend your usage capacity.

## Quick Install

1. Run `/plugin` in Claude Code
2. Go to **Marketplaces** → **Add Marketplace**
3. Enter: `shivang2000/claude-account-switcher`
4. Go to **Discover** → Install **account-switcher**
5. Restart Claude Code

## Usage

```bash
# Save current account
/account-switcher:add-account work

# List all accounts
/account-switcher:list-accounts

# Switch accounts (then restart Claude Code)
/account-switcher:switch-account personal

# Check current account
/account-switcher:whoami
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

See [account-switcher/README.md](./account-switcher/README.md) for full documentation.

## License

MIT
