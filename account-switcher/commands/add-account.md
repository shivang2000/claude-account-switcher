---
name: add-account
description: Save current Claude credentials as a named account
argument-hint: <account-name>
disable-model-invocation: true
allowed-tools: ["Read", "Write", "Bash(mkdir *)", "Bash(chmod *)", "AskUserQuestion"]
---

# Add Account - Save Current Credentials

Save the current active credentials as a named account for later switching.

**Account name**: $ARGUMENTS

## Steps

### 1. Validate Account Name

If `$ARGUMENTS` is empty, use AskUserQuestion to prompt for a name with options like "work", "personal", or "Other".

The name must be:
- 2-30 characters long
- Only letters, numbers, hyphens, and underscores
- No spaces or special characters

If invalid, show error: "Invalid account name. Use only letters, numbers, hyphens, and underscores (2-30 chars)."

### 2. Read Current Credentials

Read `~/.claude/.credentials.json`. If missing, show: "No credentials found. Please login to Claude first."

Extract from the credentials:
- `claudeAiOauth.subscriptionType`
- `claudeAiOauth.expiresAt`
- `claudeAiOauth.rateLimitTier`

### 3. Check for Existing Account

Read `~/.claude/accounts/.accounts.meta.json` if it exists. If the account name already exists, use AskUserQuestion:
- Question: "Account 'NAME' already exists. Overwrite with current credentials?"
- Options: "Yes, overwrite", "No, cancel"

### 4. Create Directory Structure

```bash
mkdir -p ~/.claude/accounts/credentials
chmod 700 ~/.claude/accounts
chmod 700 ~/.claude/accounts/credentials
```

### 5. Save Credentials

Copy current credentials to `~/.claude/accounts/credentials/<name>.json`:
- Read the full contents of `~/.claude/.credentials.json`
- Write to `~/.claude/accounts/credentials/<name>.json`
- Run: `chmod 600 ~/.claude/accounts/credentials/<name>.json`

### 6. Update Metadata

Read or create `~/.claude/accounts/.accounts.meta.json` with this structure:

```json
{
  "version": 1,
  "currentAccount": "<name>",
  "lastBackupAt": <current_timestamp_ms>,
  "accounts": {
    "<name>": {
      "addedAt": <current_timestamp_ms>,
      "lastUsedAt": <current_timestamp_ms>,
      "subscriptionType": "<from credentials>",
      "tokenExpiresAt": <from credentials>,
      "notes": ""
    }
  }
}
```

If metadata already exists, merge the new account into existing accounts object.

Set file permissions: `chmod 600 ~/.claude/accounts/.accounts.meta.json`

### 7. Confirm Success

```
## Account Saved

**Name**: <name>
**Subscription**: <subscriptionType>
**Token expires**: <formatted date>

You now have X saved accounts. Use `/account-switcher:list-accounts` to see all.
```

## Security Notes

- Credentials stored with 0600 permissions (owner read/write only)
- Directory has 0700 permissions (owner access only)
- Never log or display actual tokens
