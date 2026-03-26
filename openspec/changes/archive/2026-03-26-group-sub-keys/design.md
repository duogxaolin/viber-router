## Context

Groups currently have a single `api_key` column directly on the `groups` table. The proxy resolves group config by looking up this key. Token usage is tracked per group with optional `key_hash` for dynamic keys. There is no concept of multiple named keys per group.

For SaaS use cases, operators need to issue a unique key per end-user (customer) to track usage independently. The number of keys per group can be large (hundreds to thousands).

Current proxy flow: `parse_api_key(raw)` → `resolve_group_config(group_key)` → lookup `groups.api_key` → cache in Redis as `group:config:{api_key}`.

## Goals / Non-Goals

**Goals:**
- Allow groups to have multiple named sub-keys alongside the existing master key
- Proxy resolves sub-keys to the same group config as the master key
- Track which sub-key was used in `token_usage_logs` via `group_key_id`
- Admin CRUD API for sub-key management
- Frontend Keys tab with search, pagination, and expandable per-key usage rows
- Schema supports future quota/rate-limiting columns (nullable, not enforced)

**Non-Goals:**
- Rate limiting / quota enforcement (future phase)
- Per-key dashboard page (future phase — this phase shows usage inline via expandable rows)
- Dynamic key support for sub-keys (sub-keys use group's server default keys only)
- Auto-creation of sub-keys when creating a group
- Deletion of sub-keys (deactivate only, preserve history)

## Decisions

### 1. Master key stays on `groups` table
**Choice**: Keep `groups.api_key` as-is, add separate `group_keys` table for sub-keys.
**Why**: Zero breaking changes. Existing integrations continue working. Master key is the fast path — no JOIN needed.
**Alternative**: Move all keys to `group_keys` (cleaner model, but breaking change requiring data migration and all consumers to update).

### 2. Proxy resolution: two-step lookup
**Choice**: Try `groups.api_key` first (existing fast path), fall back to `group_keys` JOIN `groups` on miss.
**Why**: Most traffic uses master keys. Sub-key lookup only happens on cache miss for non-master keys. Both paths cache identically in Redis.
**Alternative**: Single query with UNION (simpler code, but slower for the common case).

### 3. Sub-keys do not support dynamic keys
**Choice**: When a sub-key is used, the `-rsv-` dynamic key syntax is not parsed. The entire raw header is treated as the group key for lookup.
**Why**: Sub-keys represent SaaS end-users who use the group's configured server keys. Dynamic keys are for operators who bring their own upstream keys — different use case.
**Implementation**: In `proxy_handler`, after `parse_api_key()`, if `parsed.group_key` doesn't match a master key AND `parsed.dynamic_keys` is non-empty, treat the entire raw key as a plain lookup (no dynamic key extraction).

### 4. Cache stores `group_key_id` metadata
**Choice**: Extend the cached `GroupConfig` (or a wrapper) to include `Option<Uuid>` for `group_key_id`. Master key → `None`, sub-key → `Some(id)`.
**Why**: Usage tracking needs the `group_key_id` at response time. Storing it in cache avoids a second lookup.
**Implementation**: Add `group_key_id: Option<Uuid>` to `GroupConfig`. Cache key remains `group:config:{api_key}`.

### 5. Cache invalidation covers sub-keys
**Choice**: When group config changes, query `group_keys` for all api_keys belonging to that group and invalidate each cache entry.
**Why**: Sub-keys cache the same `GroupConfig`. When servers/priorities/settings change, all cached entries (master + sub-keys) must be invalidated.
**Implementation**: Extend `invalidate_group_config` to also query and invalidate sub-key cache entries. Add `invalidate_group_all_keys(redis, db, group_id)` helper.

### 6. `token_usage_logs` gets `group_key_id` column
**Choice**: Add `group_key_id UUID NULL` column to `token_usage_logs` with an index on `(group_key_id, created_at)`.
**Why**: Direct FK enables fast per-key usage queries and JOINs with `group_keys` for name resolution. `NULL` means master key was used.

### 7. Frontend uses q-tabs to organize GroupDetailPage
**Choice**: Restructure GroupDetailPage with `q-tabs`: Properties, Servers, Keys, TTFT, Token Usage.
**Why**: Page is already long (900+ lines). Tabs improve navigation. Keys tab needs its own space for search + pagination + expandable rows given potentially thousands of keys.

### 8. Sub-key deactivation returns 403
**Choice**: Disabled sub-key → HTTP 403 "API key is disabled" (same as disabled group).
**Why**: Consistent behavior. The key exists but is not allowed — 403 is semantically correct.

## Risks / Trade-offs

- **Two-step lookup latency on cache miss**: Sub-key resolution requires an extra DB query on first use. Mitigated by Redis caching — subsequent requests are fast.
  → Mitigation: Cache TTL ensures sub-key lookups are rare after first request.

- **Cache invalidation fan-out**: Groups with many sub-keys require many Redis DEL operations on config change.
  → Mitigation: Use pipeline/batch DEL. Config changes are infrequent (admin operations).

- **Large keys table in UI**: Thousands of keys per group.
  → Mitigation: Server-side pagination, search by name, lazy-load usage on row expand.

- **Migration on partitioned table**: Adding `group_key_id` to `token_usage_logs` requires ALTER on partitioned table.
  → Mitigation: `ALTER TABLE ... ADD COLUMN` with NULL default is fast (no rewrite) on PostgreSQL.
