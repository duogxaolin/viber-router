## 1. Database Migration

- [x] 1.1 Create `viber-router-api/migrations/037_create_group_key_servers.sql` with `group_key_servers` junction table: `group_key_id` (UUID FK to `group_keys(id) ON DELETE CASCADE`), `server_id` (UUID FK to `servers(id) ON DELETE RESTRICT`), `created_at` (TIMESTAMPTZ DEFAULT now()), composite PK on `(group_key_id, server_id)`

## 2. Backend — Model

- [x] 2.1 Create `viber-router-api/src/models/group_key_server.rs` with:
  - `GroupKeyServer` struct (fields: `group_key_id: Uuid`, `server_id: Uuid`, `created_at: DateTime<Utc>`)
  - `AssignKeyServer` input struct (`server_ids: Vec<Uuid>`)
- [x] 2.2 Export `GroupKeyServer` in `viber-router-api/src/models/mod.rs`

## 3. Backend — API Routes

- [x] 3.1 Create `viber-router-api/src/routes/admin/group_key_servers.rs` with:
  - `router()` function returning `Router<AppState>` with routes:
    - `POST /` — `assign_key_servers` (accepts `AssignKeyServer` input, inserts all server assignments, 409 on duplicate, 400 if server not in group)
    - `GET /` — `list_key_servers` (returns `Vec<GroupServerDetail>` for assigned servers, maintaining `group_servers.priority` order)
    - `DELETE /{server_id}` — `remove_key_server` (deletes assignment by key_id + server_id, 404 if not found)
- [x] 3.2 Register nested router in `routes/admin/group_keys.rs` under `/{key_id}/servers` (mirrors `/{key_id}/allowed-models` pattern)
- [x] 3.3 Export new `group_key_servers` module in `routes/admin/mod.rs`
- [x] 3.4 In `assign_key_servers`: validate each `server_id` is in the group's server list (via `group_servers` join), return 400 if any server not in group. Insert all valid assignments, skip duplicates silently (or collect IDs for response). Invalidate only the sub-key's config cache via `invalidate_group_config(redis, sub_key_api_key)` ← (verify: 400 returned when assigning a server not in group, 409 on duplicate assignment, cache invalidated for sub-key only)
- [x] 3.5 In `remove_key_server`: look up sub-key API key from `group_keys` by `key_id` to call `invalidate_group_config`, return 204 on success ← (verify: 404 when assignment does not exist, cache invalidated for sub-key only)

## 4. Backend — Proxy Engine

- [x] 4.1 In `proxy.rs` `resolve_group_config`: after fetching the full group servers list (ordered by `priority`), check if `group_key_id` is `Some`. If so, query `group_key_servers` for that key_id. If rows exist, filter the servers list to only those with `server_id` in `group_key_servers` (maintaining original priority order). If no rows in `group_key_servers`, use all group servers (backward compatible) ← (verify: sub-key with assigned servers uses only those servers in priority order; sub-key with no assignments uses all group servers; master key unaffected)

## 5. Frontend — Store

- [x] 5.1 Add to `stores/groups.ts`:
  - `fetchKeyServers(groupId: string, keyId: string): Promise<GroupServerDetail[]>` — GET `/api/admin/groups/{groupId}/keys/{keyId}/servers`
  - `assignKeyServer(groupId: string, keyId: string, serverId: string): Promise<void>` — POST `/api/admin/groups/{groupId}/keys/{keyId}/servers` with `{ server_ids: [serverId] }`
  - `removeKeyServer(groupId: string, keyId: string, serverId: string): Promise<void>` — DELETE `/api/admin/groups/{groupId}/keys/{keyId}/servers/{serverId}`

## 6. Frontend — UI

- [x] 6.1 In `GroupDetailPage.vue`, inside the expanded sub-key row (`<q-tr v-if="props.expand">`), add a "Servers" section between the Subscriptions section and the SubKeyUsage component. The section shows:
  - Header: "Servers" subtitle with an "Add Server" button (only enabled if unassigned group servers exist)
  - "Add Server" button opens a dropdown menu listing group servers that are not yet assigned to this key (computed as `groupServers.filter(gs => !keyServersMap[keyId]?.some(s => s.server_id === gs.server_id))`)
  - Assigned servers shown as a table or list with columns: Server Name, Base URL, Actions (remove button with loading state)
  - Empty state: "Inherits all group servers" when no servers are assigned
- [x] 6.2 Add state: `keyServersMap: Record<string, GroupServerDetail[]>` and `keyServersLoading: Record<string, boolean>`
- [x] 6.3 Load key servers when sub-key is expanded (`onExpandSubKey`). Call `groupsStore.fetchKeyServers(groupId, keyId)` and store result in `keyServersMap[keyId]`
- [x] 6.4 "Add Server" handler: call `groupsStore.assignKeyServer(groupId, keyId, serverId)`, add returned server to `keyServersMap[keyId]` on success
- [x] 6.5 Remove handler: call `groupsStore.removeKeyServer(groupId, keyId, serverId)`, remove from `keyServersMap[keyId]` on success ← (verify: servers section appears in expanded sub-key rows, add/remove operations update UI immediately, empty state shows when no servers assigned)

## 7. Verification

- [x] 7.1 Run `cargo check` and `cargo clippy -- -D warnings` in `viber-router-api/`, fix all errors
- [x] 7.2 Run `bun run lint` and `bun run build` in frontend, fix all errors
- [x] 7.3 Run `just check` end-to-end, fix all errors
