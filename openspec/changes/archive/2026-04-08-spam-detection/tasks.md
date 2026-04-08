## 1. Database Migration

- [x] 1.1 Create `viber-router-api/migrations/035_add_content_hash_to_token_usage_logs.sql` — add `ALTER TABLE token_usage_logs ADD COLUMN IF NOT EXISTS content_hash TEXT;` and `CREATE INDEX IF NOT EXISTS idx_token_usage_logs_group_content_hash ON token_usage_logs (group_id, content_hash, created_at);` ← (verify: migration applies cleanly, index exists on the partitioned table parent)

## 2. Backend — usage_buffer.rs

- [x] 2.1 Add `pub content_hash: Option<String>` field to the `TokenUsageEntry` struct in `viber-router-api/src/usage_buffer.rs`
- [x] 2.2 Add `content_hashes: Vec<Option<String>>` to the `flush_batch` function's local vectors, populate it from `e.content_hash.clone()` in the loop, extend the INSERT query to include `content_hash` in the column list and `$15::text[]` in the UNNEST, and bind `&content_hashes`
- [x] 2.3 Update the `make_entry()` test helper in `usage_buffer.rs` to include `content_hash: None` ← (verify: `cargo check` passes, `cargo test` passes for usage_buffer tests)

## 3. Backend — proxy.rs

- [x] 3.1 In `proxy.rs`, after `body_bytes` is captured (around line 582), compute `let content_hash = Some(crate::usage_buffer::hash_key(&String::from_utf8_lossy(&body_bytes)));` — store it as a local variable accessible to the non-streaming `TokenUsageEntry` construction
- [x] 3.2 Add `content_hash: content_hash.clone()` to the non-streaming `TokenUsageEntry` struct literal (around line 1307)
- [x] 3.3 Add `content_hash: None` to the streaming `TokenUsageEntry` struct literal inside `wrap_stream_with_usage_tracking` (around line 1764) — streaming path does not have access to original body bytes ← (verify: `cargo check` passes, no clippy warnings)

## 4. Backend — spam_detection.rs

- [x] 4.1 Create `viber-router-api/src/routes/admin/spam_detection.rs` with `SpamDetectionParams` (group_id: Uuid, page: Option<i64>, limit: Option<i64>), `SpamResult` struct (group_key_id: Uuid, api_key: String, key_name: String, spam_type: String, request_count: i64, peak_rpm: i64, detected_at: DateTime<Utc>), and `ApiError` type alias following the pattern in `token_usage.rs`
- [x] 4.2 Implement `get_spam_detection` handler: require `group_id`, default page=1 limit=20; run Algorithm 1 query (low_token: input_tokens < 50, last 20 min, HAVING COUNT(*) >= 10 GROUP BY group_key_id); run Algorithm 2 query (duplicate: content_hash IS NOT NULL, last 10 min, HAVING COUNT(*) >= 10 GROUP BY group_key_id, content_hash)
- [x] 4.3 For each flagged (group_key_id, spam_type) pair, compute peak_rpm using `SELECT MAX(cnt) FROM (SELECT COUNT(*) as cnt FROM token_usage_logs WHERE group_key_id = $1 AND created_at > $2 GROUP BY date_trunc('minute', created_at)) sub` — use the appropriate detection window per spam type
- [x] 4.4 JOIN flagged keys with `group_keys` table to fetch `api_key` (full, unmasked) and `name`; set `detected_at` to `Utc::now()`; apply pagination (OFFSET/LIMIT) and return `PaginatedResponse<SpamResult>` with correct total and total_pages
- [x] 4.5 Add `pub fn router() -> Router<AppState>` returning `Router::new().route("/", get(get_spam_detection))` ← (verify: `cargo check` passes, `cargo clippy -- -D warnings` passes)

## 5. Backend — Register Module

- [x] 5.1 Add `pub mod spam_detection;` to `viber-router-api/src/routes/admin/mod.rs`
- [x] 5.2 Add `.nest("/spam-detection", spam_detection::router())` inside the `protected` router in `mod.rs` ← (verify: `cargo check` passes, endpoint is reachable behind admin auth at `/api/admin/spam-detection`)

## 6. Frontend — Store

- [x] 6.1 Add `SpamResult` TypeScript interface to `src/stores/groups.ts` with fields: `group_key_id: string`, `api_key: string`, `key_name: string`, `spam_type: 'low_token' | 'duplicate_request'`, `request_count: number`, `peak_rpm: number`, `detected_at: string`
- [x] 6.2 Add `fetchSpamDetection(groupId: string, params?: { page?: number; limit?: number })` function to the groups store — call `GET /api/admin/spam-detection` with `{ group_id: groupId, ...params }` and return the `PaginatedResponse<SpamResult>` ← (verify: TypeScript strict mode passes, `bun run lint` passes)

## 7. Frontend — GroupDetailPage.vue

- [x] 7.1 Add `<q-tab name="spam" label="Spam" />` after the "Token Usage" tab in the `q-tabs` block in `GroupDetailPage.vue`
- [x] 7.2 Add `'spam'` to the `validTabs` array
- [x] 7.3 Add spam state variables: `spamRows`, `spamLoading`, `spamError`, `spamPagination` (rowsPerPage: 20, page: 1, rowsNumber: 0) following the `subKeyPagination` pattern
- [x] 7.4 Add `spamColumns` array with columns: key (full api_key), key_name, spam_type, request_count, peak_rpm, detected_at
- [x] 7.5 Implement `loadSpam()` async function that calls `fetchSpamDetection` and updates `spamRows`, `spamPagination.rowsNumber`, handles loading/error state
- [x] 7.6 Add `onSpamRequest` handler for server-side pagination (same pattern as `onSubKeyRequest`)
- [x] 7.7 Add `if (tab === 'spam') loadSpam();` to the `watch(activeTab, ...)` handler
- [x] 7.8 Add `<q-tab-panel name="spam">` with a `q-table` (flat, bordered, dense, server-side pagination via `@request="onSpamRequest"`); add `#body-cell-spam_type` slot rendering `<q-badge :color="row.spam_type === 'low_token' ? 'orange' : 'red'" :label="row.spam_type === 'low_token' ? 'Low Token' : 'Duplicate Request'" />`; add `#body-cell-api_key` slot showing full key with `<q-btn flat dense size="xs" icon="content_copy" @click.stop="copyText(row.api_key)" />` ← (verify: `bun run lint` passes, Spam tab renders, table shows correct badge colors, copy button works)

## 8. Final Check

- [x] 8.1 Run `just check` from the workspace root and fix all lint and type errors reported for both frontend and backend ← (verify: `just check` exits 0 with no errors)
