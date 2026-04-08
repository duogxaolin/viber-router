## Context

Viber Router proxies requests to upstream LLM providers. Each request is logged in `token_usage_logs` with token counts and a hashed key identifier. Currently there is no mechanism to detect abusive patterns — specifically, keys sending many low-token requests (probing/scraping) or keys replaying identical request bodies (automated spam).

The existing `hash_key()` utility in `usage_buffer.rs` produces a 16-hex-char truncated SHA-256, which is the same pattern we will use for `content_hash`. The batch insert in `usage_buffer.rs` uses UNNEST arrays, so adding a new nullable column is a straightforward extension.

Admin routes follow a consistent pattern: a handler file under `viber-router-api/src/routes/admin/`, registered in `mod.rs`, returning `PaginatedResponse<T>` from `models/group.rs`.

## Goals / Non-Goals

**Goals:**
- Capture a truncated SHA-256 of each request body at proxy time and store it in `token_usage_logs`
- Provide a single admin endpoint that runs both spam detection algorithms and returns unified, paginated results
- Surface spam results in the admin UI with full (unmasked) API keys for quick copy-and-block

**Non-Goals:**
- Automatic blocking or rate-limiting of detected spammers (manual admin action only)
- Real-time alerting (Telegram or otherwise) for spam events
- Spam detection for non-billing endpoints (only `/v1/messages` and OpenAI-equivalent paths matter)
- Retroactive analysis of historical data before the migration

## Decisions

**Decision: content_hash computed from raw body_bytes before transformation**
The hash must reflect what the client sent, not the transformed body. `body_bytes` is captured before `transform_request_body()` is called, so hashing it there gives a stable, client-side fingerprint. Alternative (hash after transform) would conflate different clients sending the same logical request through different model mappings.

**Decision: content_hash is nullable**
Not all proxy paths reach the token usage logging code (e.g., errors before response). Making it nullable avoids forcing a hash on every code path and keeps the migration backward-compatible with existing rows.

**Decision: Single endpoint returning both algorithm results merged**
Rather than two separate endpoints, a single `/api/admin/spam-detection` endpoint runs both queries and merges results. This simplifies the frontend (one fetch, one table) and lets the admin see the full picture per group. Pagination applies to the merged result set.

**Decision: Peak RPM via date_trunc('minute', created_at)**
Using `date_trunc('minute', created_at)` to bucket requests into 1-minute windows is simple and index-friendly. An alternative (sliding window) would be more accurate but significantly more complex in SQL. The truncated-minute approach is sufficient for detecting burst patterns.

**Decision: Full API key returned (no masking)**
This is an internal admin tool. The admin needs to copy the full key to search and block spammers. Masking would defeat the purpose. This is consistent with the project's security notes.

**Decision: content_hash stored as TEXT (16 hex chars)**
Consistent with `key_hash`. Storing as TEXT avoids binary encoding complexity and keeps queries readable. 16 hex chars (64-bit prefix) is sufficient to distinguish request bodies for spam detection purposes — collision probability is negligible at the scale of 10-minute windows.

## Risks / Trade-offs

- [content_hash adds ~16 bytes per row] → Acceptable; `token_usage_logs` is already a large table and this is a small fixed overhead. The composite index `(group_id, content_hash, created_at)` will grow proportionally but is necessary for Algorithm 2 performance.
- [Streaming responses don't have body_bytes at entry creation time] → The `wrap_stream_with_usage_tracking` path creates `TokenUsageEntry` inside the stream wrapper, which does not have access to the original `body_bytes`. The `content_hash` will be `None` for streaming responses. This means Algorithm 2 will only catch non-streaming duplicate spam, which covers the most common automated abuse pattern.
- [date_trunc peak RPM is approximate] → A request at 12:00:59 and one at 12:01:01 are in different buckets despite being 2 seconds apart. This is acceptable for spam detection — we are looking for sustained bursts, not precise timing.
- [Migration on partitioned table] → `token_usage_logs` is partitioned by range on `created_at`. Adding a column with `ALTER TABLE` applies to the parent and all existing partitions. This is a standard PostgreSQL operation and should complete quickly since the column is nullable with no default computation.

## Migration Plan

1. Deploy migration `035_add_content_hash_to_token_usage_logs.sql` — adds nullable `content_hash TEXT` column and composite index. Safe to run on live table; new rows will populate the field, old rows remain NULL.
2. Deploy backend with updated `usage_buffer.rs` and `proxy.rs` — new entries will include `content_hash` for non-streaming requests.
3. Deploy frontend with new Spam tab — endpoint is available immediately after step 2.

Rollback: Drop the column and index. No data loss (the column is additive). Frontend tab can be removed independently.

## Open Questions

- Should streaming requests eventually capture content_hash? Would require passing `body_bytes` into the stream wrapper closure. Deferred — non-streaming duplicate spam is the primary concern.
- Should there be a minimum `request_count` configuration per group rather than a hardcoded threshold of 10? Deferred — hardcoded thresholds are sufficient for the initial implementation.
