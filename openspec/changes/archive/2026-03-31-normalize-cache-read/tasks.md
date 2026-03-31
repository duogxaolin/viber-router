## 1. Database Migration

- [x] 1.1 Create `viber-router-api/migrations/030_add_normalize_cache_read_to_group_servers.sql` with `ALTER TABLE group_servers ADD COLUMN normalize_cache_read BOOLEAN NOT NULL DEFAULT false` ← (verify: migration runs without errors, column exists with correct default)

## 2. Backend — Rust Structs

- [x] 2.1 Add `normalize_cache_read: bool` to `GroupServer` struct in `viber-router-api/src/models/group_server.rs`
- [x] 2.2 Add `normalize_cache_read: bool` with `#[serde(default)]` to `GroupServerDetail` struct (proxy cache struct)
- [x] 2.3 Add `normalize_cache_read: bool` to `AdminGroupServerDetail` struct
- [x] 2.4 Add `normalize_cache_read: Option<bool>` to `UpdateAssignment` struct ← (verify: all structs compile, `cargo check` passes)

## 3. Backend — calculate_cost()

- [x] 3.1 Add `normalize_cache_read: bool` parameter to `calculate_cost()` in `viber-router-api/src/subscription.rs`
- [x] 3.2 Update cache-read cost line: when `normalize_cache_read` is true, use `pricing.input_1m_usd * rate_input` instead of `pricing.cache_read_1m_usd * rate_cache_read` ← (verify: unit behavior matches spec scenarios — false uses cache_read price, true uses input price, cache_creation unaffected)

## 4. Backend — Proxy

- [x] 4.1 Update non-streaming path in `proxy.rs`: read `server.normalize_cache_read` and pass it to `calculate_cost()`
- [x] 4.2 Update `UsageTrackingStream` struct: add `normalize_cache_read: bool` field
- [x] 4.3 Update `wrap_stream_with_usage_tracking()` signature and call site: add `normalize_cache_read` parameter, pass `server.normalize_cache_read`
- [x] 4.4 Update streaming `tokio::spawn` block inside `UsageTrackingStream::poll_next`: pass `normalize_cache_read` to `calculate_cost()` ← (verify: both streaming and non-streaming paths compile and pass `normalize_cache_read` correctly)

## 5. Backend — Update Assignment Handler

- [x] 5.1 Update `update_assignment` handler in `viber-router-api/src/routes/admin/group_servers.rs` to apply `normalize_cache_read` from `UpdateAssignment` to the SQL UPDATE statement
- [x] 5.2 Update the SELECT query in `resolve_group_config` in `proxy.rs` to include `gs.normalize_cache_read` in the column list ← (verify: PUT endpoint persists the flag, GET group detail returns it, proxy query includes the column)

## 6. Backend — Public Usage API

- [x] 6.1 Update `ModelUsage` struct in `viber-router-api/src/routes/public/usage.rs`: replace `total_cache_creation_tokens: i64` and `total_cache_read_tokens: i64` with `effective_input_tokens: i64`
- [x] 6.2 Update the SQL query in the public usage handler to compute `effective_input_tokens = SUM(input_tokens) + COALESCE(SUM(cache_creation_input_tokens), 0) + COALESCE(SUM(cache_read_input_tokens), 0)` and remove the separate cache column aggregations ← (verify: response shape matches spec — only model, effective_input_tokens, total_output_tokens, request_count, cost_usd; NULL cache tokens coalesce to 0)

## 7. Frontend — Store Interface

- [x] 7.1 Add `normalize_cache_read: boolean` to `GroupServerDetail` interface in `src/stores/groups.ts`
- [x] 7.2 Add `normalize_cache_read?: boolean` to the `updateAssignment` input type in `src/stores/groups.ts`

## 8. Frontend — GroupDetailPage Cost Rates Modal

- [x] 8.1 Add `normalize_cache_read` to the `rateForm` reactive object (initialized from `s.normalize_cache_read`)
- [x] 8.2 Add a `q-toggle` for "Normalize Cache Read" inside the Cost Rates modal template in `src/pages/GroupDetailPage.vue`
- [x] 8.3 Update `openRateModal()` to populate `rateForm.normalize_cache_read` from the server object
- [x] 8.4 Update `onSaveRates()` to include `normalize_cache_read: rateForm.value.normalize_cache_read` in the `updateAssignment` call ← (verify: toggle appears in modal, saving persists the value, modal re-opens with correct state after save)

## 9. Frontend — PublicUsagePage

- [x] 9.1 Update `ModelUsage` interface in `src/pages/PublicUsagePage.vue`: replace `total_cache_creation_tokens` and `total_cache_read_tokens` with `effective_input_tokens: number`
- [x] 9.2 Update the usage table columns definition: remove Cache Creation and Cache Read columns, update Input column to use `effective_input_tokens` field ← (verify: table shows exactly Model, Input, Output, Requests, Cost columns; Input displays effective_input_tokens value)

## 10. Final Check

- [x] 10.1 Run `just check` and fix all type errors and lint errors reported for both frontend and backend
