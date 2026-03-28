## Why

There is no visibility into per-server or chain-level uptime based on real proxy traffic. Admins cannot see which servers are healthy or degraded over time, and end-users on the public usage page have no indication of service status. Uptime tracking derived from actual `/v1/*` requests provides ground-truth reliability data.

## What Changes

- Add a new `uptime_checks` partitioned table to record every server attempt in the proxy failover chain (group_id, server_id, status_code, latency_ms, request_id, created_at)
- Add an async `uptime_buffer` module using the existing mpsc batch-flush pattern (same as ttft_buffer, log_buffer, usage_buffer)
- Instrument the proxy handler to emit an uptime check entry for each server attempt, with a shared `request_id` UUID per request to link attempts in the same chain
- Add admin API endpoint `GET /api/admin/groups/{id}/uptime` returning per-server status bucketed into 90 × 30-minute intervals
- Add public API endpoint `GET /api/public/uptime?key=...` returning chain-level status bucketed into 90 × 30-minute intervals (chain success = any server in the request returned 2xx)
- Display per-server status bars under each server in the GroupDetailPage servers tab
- Display chain-level status text + status bars in PublicUsagePage

## Capabilities

### New Capabilities
- `uptime-data-collection`: Proxy instrumentation, uptime_checks table, uptime_buffer async flush, partition management
- `uptime-admin-api`: Admin API endpoint for per-server uptime bars data
- `uptime-public-api`: Public API endpoint for chain-level uptime bars data
- `uptime-admin-ui`: Per-server status bars in GroupDetailPage servers tab
- `uptime-public-ui`: Chain-level status text and bars in PublicUsagePage

### Modified Capabilities
- `server-setup`: Add uptime_tx mpsc channel to AppState, spawn uptime_buffer flush task, ensure partitions for uptime_checks table
- `proxy-engine`: Emit uptime check entries for each server attempt in the failover chain

## Impact

- **Database**: New migration for `uptime_checks` partitioned table with indexes on (group_id, server_id, created_at) and (group_id, created_at)
- **Backend**: New `uptime_buffer.rs` module, new `routes/admin/uptime.rs` and `routes/public/uptime.rs`, modified `proxy.rs` (emit entries), modified `main.rs` (channel + flush task + partitions), modified `routes/mod.rs` (AppState + routing)
- **Frontend**: Modified `GroupDetailPage.vue` (status bars under servers), modified `PublicUsagePage.vue` (status section), possible new `UptimeBars.vue` shared component
- **Dependencies**: No new crate or npm dependencies expected
- **Config**: No new env vars (uses existing LOG_RETENTION_DAYS for partition cleanup)
