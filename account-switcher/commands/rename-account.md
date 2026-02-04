---
name: rename-account
description: Rename an existing saved account
argument-hint: <old-name> <new-name>
disable-model-invocation: true
allowed-tools: ["Read", "Write", "Bash(mv *)", "AskUserQuestion"]
---

# Rename Account

Rename a saved account without losing any data.

**Arguments**: $ARGUMENTS

Expected format: `<old-name> <new-name>`

## Steps

### 1. Parse Arguments

Split `$ARGUMENTS` by space to get old-name and new-name.

If not provided or incomplete:
- Use AskUserQuestion for old name: "Which account do you want to rename?"
- Then ask for new name: "What should the new name be?"

### 2. Validate Old Name Exists

Read `~/.claude/accounts/.accounts.meta.json`.

If old-name not found: "Account 'OLD-NAME' not found. Use `/account-switcher:list-accounts` to see available accounts."

### 3. Validate New Name

New name must:
- Be 2-30 characters long
- Only letters, numbers, hyphens, and underscores
- Not already exist in accounts

If new name already exists: "Account 'NEW-NAME' already exists. Choose a different name."

If invalid format: "Invalid account name. Use only letters, numbers, hyphens, and underscores (2-30 chars)."

### 4. Rename Credentials File

```bash
mv ~/.claude/accounts/credentials/<old-name>.json ~/.claude/accounts/credentials/<new-name>.json
```

### 5. Update Metadata

Read `.accounts.meta.json`:
1. Copy the account data from old key to new key
2. Delete the old key
3. If `currentAccount` was the old name, update it to new name
4. Write updated metadata

### 6. Confirm

```
## Account Renamed

**Old name**: <old-name>
**New name**: <new-name>

All settings and credentials preserved.
```

## Edge Cases

- Renaming current account: Update `currentAccount` field in metadata
- Credentials file missing: Show error, don't update metadata
