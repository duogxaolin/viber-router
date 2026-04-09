## Context

Viber Router routes API requests through group-defined server chains with priority ordering. Groups have a master key and optional sub-keys with independent rate limits. Currently, sub-keys always route through all enabled servers in the group chain. There is no mechanism to restrict a sub-key to a subset of those servers.

The existing data model follows a junction-table pattern (e.g., `group_key_allowed_models`). The existing `group_key_allowed_models` junction table is the direct template for `group_key_servers`. The proxy resolves `GroupConfig` per API key from Redis (with DB fallback), and cache is invalidated via `invalidate_group_config(api_key)`.

## Goals / Non-Goals

**Goals:**
- Allow admins to assign specific servers from the group's server chain to a sub-key.
- When servers are assigned, proxy routing for that sub-key uses only those servers, in the same priority order defined by `group_servers.priority`.
- When no servers are assigned to a sub-key, it inherits the full group server chain (backward compatible with existing behavior).
- Targeted cache invalidation: only the affected sub-key's config cache is invalidated on assignment changes.

**Non-Goals:**
- Assigning servers to a sub-key that are not part of the group chain (only servers already in the group can be assigned).
- Changing server priority per key (priority is always driven by `group_servers.priority`).
- Assigning servers to the master key (master key always uses all group servers).

## Decisions

### 1. Junction table with CASCADE delete on group_key_id

**Decision**: `group_key_servers` has a composite PK on `(group_key_id, server_id)` with `ON DELETE CASCADE` on `group_key_id`. `server_id` has `ON DELETE RESTRICT`.

**Rationale**: Mirrors the `group_key_allowed_models` pattern exactly. When a sub-key is deleted, its server assignments must be cleaned up automatically. When a server is deleted from the database, assignments must be blocked if any sub-key depends on it (restrict prevents accidental data inconsistency).

### 2. Server filtering at `resolve_group_config` build time

**Decision**: Server filtering is applied when building the `GroupConfig` struct in `resolve_group_config`, not in the proxy failover loop.

**Rationale**: The `GroupConfig` struct already contains the full `servers: Vec<GroupServerDetail>` list. Adding a filter at build time means the proxy failover loop iterates over the correct list with no changes needed there. It also means the restriction is enforced at config resolution time, so a single cache entry always contains the correct server set for that key.

### 3. Targeted invalidation via `invalidate_group_config`

**Decision**: Assigning or removing a server from a sub-key calls `invalidate_group_config(redis, sub_key_api_key)`, not `invalidate_group_all_keys`.

**Rationale**: Unlike group-level changes (e.g., adding a server to a group, which affects all sub-keys), a per-key server assignment only affects one sub-key. Using targeted invalidation avoids unnecessary cache churn across unrelated sub-keys. The sub-key's API key is looked up from `group_keys` by `key_id`.

### 4. Frontend server selector shows only unassigned group servers

**Decision**: The "Add Server" dropdown in the expanded sub-key row shows only servers that are in the group but not yet assigned to the key.

**Rationale**: The `GroupDetailPage` already has access to the group's server list (for the Servers tab) and the key's assigned servers (fetched per expand). The available servers are the set difference: `groupServers - keyAssignedServers`. No separate API endpoint is needed for available servers.

### 5. No new `GroupConfig` field for key servers

**Decision**: Do not add a `key_servers` or `assigned_server_ids` field to `GroupConfig`. The server list in `GroupConfig` is the definitive, filtered list after applying key-level restrictions.

**Rationale**: Adding a separate field would require dual maintenance in the proxy loop. The filtered `servers` list is sufficient — if `group_key_id` is `Some` and `group_key_servers` has rows, those rows determine which servers are included, maintaining their original `group_servers.priority` order.

## Risks / Trade-offs

- **[Risk] Race condition on concurrent assign**: Two admins simultaneously assign the same server to the same key. **Mitigation**: The composite primary key enforces uniqueness at the DB level. The route handler catches the duplicate-key error and returns HTTP 409 Conflict (same pattern as `group_key_allowed_models`).

- **[Risk] Orphaned `group_key_servers` if server removed from group**: If an admin removes a server from `group_servers`, any `group_key_servers` referencing it becomes invalid. **Mitigation**: `server_id` has `ON DELETE RESTRICT`, so the DB refuses to delete a server if any sub-key assignments reference it. The admin must remove those assignments first.

- **[Risk] Cache not invalidated for the master key when sub-key assignments change**: Sub-key assignments don't affect master-key routing. **Mitigation**: `invalidate_group_config` is called with the sub-key's API key only, which is correct behavior — the master key config is unaffected.

- **[Trade-off] Frontend polling on expand**: The "Servers" section requires a per-key API call on sub-key expand. This is acceptable because sub-key expansion is a deliberate user action, not a page load requirement. The servers section is only shown if there are group servers.

## Migration Plan

1. Apply migration `037_create_group_key_servers.sql` — zero-downtime, adds a new table only.
2. Deploy backend with new model, routes, and `proxy.rs` changes.
3. Deploy frontend with updated `groups.ts` store and `GroupDetailPage.vue`.
4. No rollback needed for the table — the column addition is backward compatible. If a rollback is needed, the new routes and frontend sections are simply not used.

## Open Questions

- Should deleting a server from `group_servers` automatically clean up `group_key_servers` assignments? (Current design uses RESTRICT to prevent accidental deletion; admin must explicitly remove key assignments first. This is a conservative choice. An alternative is CASCADE with a warning. We stick with RESTRICT.)
