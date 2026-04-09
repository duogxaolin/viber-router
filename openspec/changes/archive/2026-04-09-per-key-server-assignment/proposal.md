## Why

Currently, all sub-keys in a group inherit the full server chain of their parent group. There is no way to restrict a sub-key to use only a subset of the group's servers. This limits the ability to offer differentiated tiers of service (e.g., a "basic" sub-key that only routes to a single budget server vs. a "pro" sub-key that routes across all servers). Per-key server assignment fills this gap.

## What Changes

- **Database**: New `group_key_servers` junction table linking sub-keys to specific servers.
- **Backend API**: Three new REST endpoints to assign, list, and remove server assignments for a sub-key.
- **Proxy engine**: When resolving a sub-key's config, filter the server list to only assigned servers if any are defined; otherwise use the full group server chain (backward compatible).
- **Cache invalidation**: When a sub-key's server assignments change, invalidate only that sub-key's cache entry.
- **Frontend**: "Servers" section in expanded sub-key rows in the Group Detail page, showing assigned servers and an "Add Server" dropdown of available group servers.

## Capabilities

### New Capabilities

- `per-key-server-assignment`: Allows admins to assign specific servers to a sub-key. When servers are assigned, that sub-key can only use those servers (maintaining group priority order). If no servers are assigned, the sub-key inherits all group servers. Proxy routing respects these restrictions.

### Modified Capabilities

- `config-cache`: The write-through cache invalidation spec needs a new scenario added for sub-key server assignment changes — invalidating only the affected sub-key's cache entry (not all group keys).

## Impact

- **Database**: New migration `037_create_group_key_servers.sql`.
- **Backend**: New `models/group_key_server.rs`, new `routes/admin/group_key_servers.rs`, nested router registration in `group_keys.rs`, `resolve_group_config` in `proxy.rs` extended with server filtering logic.
- **Frontend**: `stores/groups.ts` extended with three new store functions; `GroupDetailPage.vue` gets a "Servers" section inside expanded sub-key rows.
- **Dependencies**: No new external dependencies. Uses existing `sqlx`, `axum`, `Redis` pool.
