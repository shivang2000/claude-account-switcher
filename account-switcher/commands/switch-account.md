---
name: switch-account
description: Switch to a different saved Claude account (requires restart)
argument-hint: [account-name]
disable-model-invocation: true
allowed-tools: ["Read", "Write", "Bash(cp *)", "Bash(chmod *)", "Bash(date *)", "AskUserQuestion"]
---

# Switch Account

Switch to a different saved Claude account. Requires restart after switching.

**Target account**: $ARGUMENTS

## Steps

### 1. Load Available Accounts

Read `~/.claude/accounts/.accounts.meta.json`.

If missing or empty:
```
No saved accounts found.

Use `/account-switcher:add-account <name>` to save your first account.
```

### 2. Select Account

**If `$ARGUMENTS` is provided**:
- Check if account exists in metadata
- If not found: "Account 'NAME' not found. Available accounts: work, personal, ..."

**If `$ARGUMENTS` is empty** (interactive mode):
- Use AskUserQuestion with a list of available accounts
- Include token status in options: "work (pro) - Valid", "personal (pro) - Warning"

### 3. Check Token Status and Warn

Calculate token status for selected account.

If token is expired or expiring within 24 hours, use AskUserQuestion:
- Question: "The token for 'NAME' is [EXPIRED/expiring soon]. You may need to re-login after switching. Continue?"
- Options: "Yes, switch anyway", "No, cancel"

### 4. Validate Target Credentials Exist

Check that `~/.claude/accounts/credentials/<name>.json` exists and is valid JSON.

If missing: "Credentials file for 'NAME' is missing. Account may be corrupted. Use `/account-switcher:remove-account NAME` to clean up."

### 5. Check Current Credentials Saved

Read current metadata to see if currentAccount is set and matches a saved account.

If current credentials are NOT saved (currentAccount not in accounts or doesn't match), use AskUserQuestion:
- Question: "Current credentials are not saved. Save them before switching?"
- Options: "Yes, save as...", "No, proceed (current credentials will be lost)"

If user wants to save, prompt for name and follow add-account logic first.

### 6. Create Backup

Before any switch, create a timestamped backup:
- Read current `~/.claude/.credentials.json`
- Write to `~/.claude/accounts/.credentials.backup.json`
- `chmod 600 ~/.claude/accounts/.credentials.backup.json`

### 7. Perform Switch

1. Read target credentials from `~/.claude/accounts/credentials/<name>.json`
2. Write to `~/.claude/.credentials.json`
3. Run: `chmod 600 ~/.claude/.credentials.json`

### 8. Update Metadata

Update `~/.claude/accounts/.accounts.meta.json`:
- Set `currentAccount` to new account name
- Update `lastUsedAt` for the new account
- Update `lastBackupAt` timestamp

### 9. Confirm and Instruct

```
## Account Switched

**Switched to**: <name>
**Previous**: <old-name> (backed up)
**Subscription**: <subscriptionType>
**Token status**: <status>

## RESTART REQUIRED

Claude Code loads credentials at startup. Please restart to use the new account:

1. Exit this session (Ctrl+C or type 'exit')
2. Run `claude` again

Your previous credentials are backed up at:
~/.claude/accounts/.credentials.backup.json
```

## Safety Features

- Always creates backup before switching
- Warns about unsaved current credentials
- Warns about expired tokens before switching
- Validates target credentials exist before switching
