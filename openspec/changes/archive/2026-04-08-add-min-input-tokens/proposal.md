## Why

The existing `max_input_tokens` threshold lets admins skip servers when a request is too large, but there is no way to reserve a server exclusively for large requests. Adding `min_input_tokens` closes this gap: admins can now route small requests away from servers that are optimized (or priced) for high-token workloads.

## What Changes

- New nullable `min_input_tokens` column on `group_servers` (database migration).
- Backend models updated to carry the new field through the data layer.
- Proxy routing gate: skip a server when `estimated_tokens < min_input_tokens` (mirrors the existing `max_input_tokens` gate).
- Admin API: `assign_server` and `update_assignment` endpoints accept and persist the new field.
- Frontend group-detail page: new number input and badge for `min_input_tokens`, matching the existing `max_input_tokens` UI.

## Capabilities

### New Capabilities

- `min-input-token-threshold`: Per-server minimum input token gate — skip a server in the failover waterfall when the estimated token count falls below the configured minimum.

### Modified Capabilities

- `max-input-token-threshold`: The token-threshold spec now covers both an upper bound (`max_input_tokens`) and a lower bound (`min_input_tokens`). The estimation algorithm and fail-open semantics are unchanged; only the gate logic is extended.

## Impact

- **Database**: one new nullable integer column; requires a migration.
- **Backend**: `group_server.rs` models, `admin/group_servers.rs` handlers, `proxy.rs` routing logic.
- **Frontend**: `src/stores/groups.ts` type definitions, `src/pages/GroupDetailPage.vue` UI.
- **No breaking changes**: the column is nullable and defaults to NULL, so existing server configurations are unaffected.
