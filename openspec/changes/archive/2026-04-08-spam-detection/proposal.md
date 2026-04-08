## Why

Competitors are abusing the API proxy by spamming requests with very low token counts or sending identical request bodies repeatedly, inflating costs and degrading service quality. Admins need a way to detect and act on these patterns quickly.

## What Changes

- Add `content_hash` column to `token_usage_logs` to capture a truncated SHA-256 of each request body
- Compute and store `content_hash` in the proxy pipeline at request time
- Add a new admin endpoint `GET /api/admin/spam-detection` that runs two detection algorithms and returns flagged keys with full (unmasked) API keys for immediate action
- Add a "Spam" tab to the Group Detail page in the admin UI with a paginated table of flagged keys

## Capabilities

### New Capabilities

- `spam-detection-api`: Backend endpoint that detects low-token spam and duplicate-request spam per group, returning flagged keys with peak RPM and full API key
- `spam-detection-ui`: Frontend "Spam" tab in GroupDetailPage showing paginated spam results with copy-to-clipboard for API keys

### Modified Capabilities

- `token-usage-storage`: Add `content_hash` field to `token_usage_logs` table and to the `TokenUsageEntry` struct and batch insert logic

## Impact

- New migration file: `viber-router-api/migrations/035_add_content_hash_to_token_usage_logs.sql`
- Modified: `viber-router-api/src/usage_buffer.rs` (new field + updated insert)
- Modified: `viber-router-api/src/routes/proxy.rs` (compute content_hash from body bytes)
- New: `viber-router-api/src/routes/admin/spam_detection.rs`
- Modified: `viber-router-api/src/routes/admin/mod.rs` (register new module)
- Modified: `src/stores/groups.ts` (add fetchSpamDetection)
- Modified: `src/pages/GroupDetailPage.vue` (add Spam tab + table)
