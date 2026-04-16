## Why

Admins need to restrict certain proxy servers to specific time windows — for example, limiting a server to business hours or a regional provider to its daytime availability. Without active hours support, servers are always eligible in the failover chain, which can route traffic to servers that are expensive, rate-limited, or unreliable outside certain periods.

## What Changes

- Add 3 nullable columns to `group_servers`: `active_hours_start`, `active_hours_end`, `active_hours_timezone` (TEXT, all nullable, default NULL = 24/7)
- Add a new skip condition (#11) in the proxy failover loop: skip servers whose current local time is outside their configured active window
- Add overnight window support (when start > end, the window wraps midnight)
- Fail-open: incomplete config (partial fields set) or unparseable timezone → treat as 24/7
- Add `chrono-tz` crate dependency for IANA timezone parsing
- Expose the 3 fields in admin API structs (`GroupServer`, `GroupServerDetail`, `AdminGroupServerDetail`, `UpdateAssignment`)
- Add "Active Hours" section to the Edit Server Dialog in the frontend (`GroupDetailPage.vue`)
- Display active hours badge on server cards in the group detail view

## Capabilities

### New Capabilities

- `server-active-hours`: Configure per-group-server active time windows with IANA timezone support; servers outside their window are automatically skipped during proxy failover

### Modified Capabilities

- `group-server-assignment`: Admin API `UpdateAssignment` struct gains 3 new optional fields; existing assignments with NULL values continue to behave as 24/7
- `proxy-engine`: Failover loop gains a new skip condition (active hours check) evaluated at request time using cached config data

## Impact

- **Database**: Migration 039 — 3 new nullable TEXT columns on `group_servers`
- **Backend crates**: `chrono-tz` added to `viber-router-api/Cargo.toml`
- **Backend files**: `viber-router-api/src/models.rs` (or equivalent model file), `viber-router-api/src/routes/proxy.rs`, `viber-router-api/src/routes/group_servers.rs`
- **Frontend files**: `src/pages/GroupDetailPage.vue`
- **Cache**: Active hours config is stored in `GroupConfig` in Redis; updating via admin API triggers `invalidate_group_all_keys()` as usual
