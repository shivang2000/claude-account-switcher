---
name: list-accounts
description: List all saved Claude accounts with status and expiration warnings
disable-model-invocation: true
allowed-tools: ["Read", "Bash(ls *)", "Bash(date *)"]
---

# List Saved Accounts

Display all saved accounts with their status, subscription type, and token health.

## Steps

### 1. Read Metadata File

Read `~/.claude/accounts/.accounts.meta.json`.

If the file doesn't exist or accounts directory doesn't exist:
```
No accounts saved yet.

Use `/account-switcher:add-account <name>` to save your first account.
```

### 2. Read Current Credentials

Read `~/.claude/.credentials.json` to identify which account is currently active.
Compare with the `currentAccount` field in metadata.

### 3. Calculate Token Status for Each Account

For each account in metadata, calculate token status based on `tokenExpiresAt`:
- **Valid**: expiresAt > now + 24 hours → show "(X days)"
- **Warning**: expiresAt within 24 hours → show "Warning (X hours)"
- **Expired**: expiresAt < now → show "EXPIRED"

### 4. Display Formatted Table

```
## Saved Accounts

| Status | Name       | Type | Token Status          | Last Used    |
|--------|------------|------|-----------------------|--------------|
| active | work       | pro  | Valid (3 days)        | Now          |
|        | personal   | pro  | Warning (12 hours)    | 2 days ago   |
|        | testing    | pro  | EXPIRED               | 5 days ago   |

**Legend**:
- "active" = Currently active account
- "Warning" = Token expires within 24 hours
- "EXPIRED" = Token needs refresh (may require re-login after switch)

**Commands**:
- `/account-switcher:switch-account` - Switch to a different account
- `/account-switcher:whoami` - Show current account details
- `/account-switcher:add-account <name>` - Save current credentials
```

## Edge Cases

- Empty accounts: Show helpful message to add first account
- Missing credential files in `credentials/` directory: Mark as "corrupted" in list
- Current credentials don't match any saved account: Show "(unsaved)" indicator
