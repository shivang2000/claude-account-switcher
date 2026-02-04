---
name: whoami
description: Show which Claude account is currently active and token status
disable-model-invocation: true
allowed-tools: ["Read", "Bash(date *)"]
---

# Who Am I - Current Account Status

Display information about the currently active Claude account.

## Steps

1. **Read current credentials** from `~/.claude/.credentials.json`
   - Extract: subscriptionType, expiresAt, scopes, rateLimitTier
   - Calculate time until token expiration

2. **Read metadata** from `~/.claude/accounts/.accounts.meta.json` (if exists)
   - Find which saved account name matches currentAccount field
   - If metadata doesn't exist, show "Unknown - not saved"

3. **Calculate token status**:
   - Get current time in milliseconds
   - Compare to expiresAt timestamp
   - Show "Valid (X days)" or "Warning (X hours)" or "EXPIRED"

## Output Format

Display a formatted summary:

```
## Current Account

**Account**: [name or "Unknown - not saved"]
**Subscription**: [subscriptionType]
**Rate Limit Tier**: [rateLimitTier]
**Token Status**: [Valid/Warning/EXPIRED] ([time remaining])

Use `/account-switcher:list-accounts` to see all saved accounts.
Use `/account-switcher:add-account <name>` to save this account.
```

## Edge Cases

- If `~/.claude/.credentials.json` doesn't exist: "No credentials found. Please login first."
- If metadata file missing: Show "Unknown account" but display credential info
- If token expired: "Token EXPIRED - Claude Code will attempt to refresh on restart"
