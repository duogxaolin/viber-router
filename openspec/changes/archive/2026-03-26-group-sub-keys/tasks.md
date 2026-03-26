## 1. Database

- [x] 1.1 Create migration 013: `group_keys` table (id, group_id FK, api_key UNIQUE, name, is_active, monthly_token_limit, monthly_request_limit, created_at, updated_at) with indexes on api_key and group_id
- [x] 1.2 Create migration 014: Add `group_key_id UUID NULL` column to `token_usage_logs` partitioned table, add index on `(group_key_id, created_at)` ← (verify: migrations run without errors, schema matches design.md)

## 2. Backend Models

- [x] 2.1 Create `GroupKey` struct in `viber-router-api/src/models/group_key.rs` with all fields, plus `CreateGroupKey`, `UpdateGroupKey` DTOs
- [x] 2.2 Add `group_key_id: Option<Uuid>` to `GroupConfig` in `viber-router-api/src/models/group_server.rs`
- [x] 2.3 Add `group_key_id: Option<Uuid>` to `TokenUsageEntry` in `viber-router-api/src/usage_buffer.rs` and update `flush_batch` to include it in the INSERT
- [x] 2.4 Register `group_key` module in `viber-router-api/src/models/mod.rs` ← (verify: cargo check passes, all new structs accessible)

## 3. Admin API — Sub-key CRUD

- [x] 3.1 Create `viber-router-api/src/routes/admin/group_keys.rs` with router and handlers: list (paginated + search), create, update, regenerate
- [x] 3.2 Register group_keys routes under `/api/admin/groups/:group_id/keys` in admin router
- [x] 3.3 Add `group_key_id` filter support to `token_usage.rs` GET endpoint ← (verify: all CRUD endpoints work, pagination and search correct, token-usage filter works)

## 4. Proxy Resolution

- [x] 4.1 Update `resolve_group_config` in `proxy.rs`: on master key miss, fall back to `group_keys` JOIN `groups` lookup, set `group_key_id` on `GroupConfig`
- [x] 4.2 Add sub-key `is_active` check in proxy_handler — return 403 if sub-key disabled
- [x] 4.3 Handle sub-key + dynamic key conflict: when master key miss and parsed.dynamic_keys is non-empty, re-lookup using entire raw key as plain key
- [x] 4.4 Pass `group_key_id` from `GroupConfig` to `TokenUsageEntry` in both SSE and non-SSE response paths ← (verify: proxy resolves sub-keys correctly, disabled sub-key returns 403, usage logs contain group_key_id)

## 5. Cache Invalidation

- [x] 5.1 Add `invalidate_group_all_keys(redis, db, group_id)` helper in `cache.rs` that queries `group_keys` for all api_keys and invalidates each + the master key
- [x] 5.2 Update all group config change paths (update group, assign/remove/reorder server, delete group) to use `invalidate_group_all_keys` instead of single-key invalidation
- [x] 5.3 Update `invalidate_groups_by_server` to also invalidate sub-key cache entries for affected groups ← (verify: cache invalidation covers master + all sub-keys on any group config change)

## 6. Frontend — Store

- [x] 6.1 Add `GroupKey` interface and sub-key API methods to `src/stores/groups.ts`: fetchGroupKeys, createGroupKey, updateGroupKey, regenerateGroupKey, fetchKeyUsage

## 7. Frontend — GroupDetailPage Tabs

- [x] 7.1 Restructure GroupDetailPage.vue with `q-tabs` / `q-tab-panels`: Properties (existing cards), Servers, Keys, TTFT, Token Usage
- [x] 7.2 Implement Keys tab: search input, "Create Key" button, paginated q-table with columns (Name, Key masked + copy, Status toggle, Actions)
- [x] 7.3 Implement expandable rows: on expand, fetch per-key usage and display nested table (Server, Model, Input, Output, Cache Creation, Cache Read, Requests)
- [x] 7.4 Implement create key dialog, regenerate confirmation dialog, and active toggle handler ← (verify: tabs work, keys CRUD functional, expandable usage rows load correctly, pagination and search work)

## 8. Verification

- [x] 8.1 Run `just check` — fix all type errors and lint issues ← (verify: zero errors from cargo clippy, cargo check, bun run lint)
