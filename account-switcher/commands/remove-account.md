---
name: remove-account
description: Remove a saved Claude account
argument-hint: <account-name>
disable-model-invocation: true
allowed-tools: Read, Write, Bash(rm *), AskUserQuestion
---

# Remove Account

Delete a saved account from storage.

**Account to remove**: $ARGUMENTS

## Steps

### 1. Validate Arguments

If `$ARGUMENTS` is empty, use AskUserQuestion to select from available accounts (excluding current active account).

### 2. Check Account Exists

Read `~/.claude/accounts/.accounts.meta.json`.

If account not found: "Account 'NAME' not found. Use `/account-switcher:list-accounts` to see available accounts."

### 3. Prevent Removing Active Account

Read metadata and check if `currentAccount` equals the account being removed.

If trying to remove currently active account:
```
Cannot remove the currently active account.

Switch to a different account first:
  /account-switcher:switch-account <other-account>
```

### 4. Confirm Deletion

Use AskUserQuestion:
- Question: "Are you sure you want to remove account 'NAME'? This cannot be undone."
- Options: "Yes, remove permanently", "No, cancel"

### 5. Remove Files

If confirmed:
1. Delete credentials file: `rm ~/.claude/accounts/credentials/<name>.json`
2. Update metadata: Remove the account entry from `accounts` object
3. Write updated metadata back to `.accounts.meta.json`

### 6. Confirm Removal

```
## Account Removed

**Removed**: <name>

Remaining accounts: work, personal, ...

Use `/account-switcher:list-accounts` to see your accounts.
```

## Edge Cases

- Last account: Allow removal but warn that no accounts will remain
- Credentials file already missing: Still remove from metadata, note that file was already gone
