## 1. Database Migration

- [x] 1.1 Create migration for `uptime_checks` partitioned table (id UUID PK, created_at TIMESTAMPTZ, group_id UUID NOT NULL, server_id UUID NOT NULL, status_code SMALLINT NOT NULL, latency_ms INTEGER NOT NULL, request_id UUID NOT NULL) with indexes on (group_id, server_id, created_at) and (group_id, created_at) ← (verify: migration runs without errors, table is partitioned by range on created_at, indexes exist)

## 2. Uptime Buffer

- [x] 2.1 Create `uptime_buffer.rs` with `UptimeCheckEntry` struct (group_id, server_id, status_code i16, latency_ms i32, request_id, created_at) and `flush_task` function following the same mpsc batch-flush pattern as `ttft_buffer.rs`
- [x] 2.2 Add `uptime_tx: mpsc::Sender<UptimeCheckEntry>` to `AppState` in `routes/mod.rs`
- [x] 2.3 Create the mpsc channel (capacity 10,000), spawn the uptime flush task, and ensure partitions for `uptime_checks` in `main.rs`. Add `uptime_checks` to daily partition maintenance and graceful shutdown ← (verify: channel created, flush task spawned, partitions ensured on startup, daily maintenance includes uptime_checks, shutdown drains buffer)

## 3. Proxy Instrumentation

- [x] 3.1 Add `mod uptime_buffer` to `main.rs`
- [x] 3.2 Add `emit_uptime_entry` helper function in `proxy.rs` (similar to `emit_ttft_entry`)
- [x] 3.3 Generate `request_id = Uuid::new_v4()` at the start of `proxy_handler`
- [x] 3.4 Emit uptime check entry for each server attempt: after count-tokens default server attempt, after each failover chain server attempt (success, failover status, connection error, TTFT timeout, empty stream) ← (verify: every code path that tries a server emits an uptime entry, all entries share the same request_id, status_code=0 for connection errors and timeouts)

## 4. Admin Uptime API

- [x] 4.1 Create `routes/admin/uptime.rs` with `GET /` handler that queries `uptime_checks` for a group, aggregates into 90 × 30-minute buckets per server, returns `{ servers: [{ server_id, server_name, buckets: [{ timestamp, total, success }] }] }`
- [x] 4.2 Register the uptime route in `routes/admin/groups.rs` router as `/{id}/uptime` ← (verify: endpoint returns correct per-server bucketed data, 90 buckets covering ~45 hours, success = status_code 200-299)

## 5. Public Uptime API

- [x] 5.1 Create `routes/public/uptime.rs` with `GET /` handler that validates sub-key, queries `uptime_checks` for the group, aggregates chain-level success by request_id into 90 × 30-minute buckets, returns `{ status, uptime_percent, buckets: [{ timestamp, total_requests, successful_requests }] }`
- [x] 5.2 Register the uptime route in `routes/public/mod.rs` ← (verify: endpoint returns correct chain-level data, status text matches spec thresholds, rate limiting and key validation work)

## 6. Frontend: UptimeBars Component

- [x] 6.1 Create `src/components/UptimeBars.vue` — shared component accepting `buckets` array prop `{ timestamp, total, success }[]`, renders 90 colored bars (green >95%, yellow 50-95%, red <50%, gray no data), with hover tooltip showing time range + stats, and ARIA labels for accessibility

## 7. Frontend: Group Detail Page

- [x] 7.1 Add uptime data fetching to `GroupDetailPage.vue` — call `GET /api/admin/groups/{id}/uptime` on mount, store per-server uptime data
- [x] 7.2 Render `UptimeBars` under each server in the servers tab list, with error state "Unable to load status" + retry button ← (verify: bars appear under each server, colors match spec, tooltip works, error state shows retry button)

## 8. Frontend: Public Usage Page

- [x] 8.1 Add uptime data fetching to `PublicUsagePage.vue` — call `GET /api/public/uptime?key=...` alongside existing usage fetch, include in auto-refresh
- [x] 8.2 Add "Status" section above subscriptions with status badge (Operational/Degraded/Down/No data with appropriate colors) and `UptimeBars` component, with error state "Unable to load status" + retry button ← (verify: status badge color matches spec, bars render correctly, error state works, auto-refresh includes uptime)

## 9. Unit Tests

- [x] 9.1 Add unit tests to `uptime_buffer.rs` — buffer overflow drops entry, send succeeds when not full, channel close signals receiver (same pattern as ttft_buffer tests) ← (verify: all tests pass, `cargo check` and `cargo clippy -- -D warnings` pass)

## 10. Final Check

- [x] 10.1 Run `just check` to verify type-check + lint for both frontend and backend pass with zero errors
