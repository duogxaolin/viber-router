## Context

Viber Router routes API requests through a per-group failover waterfall of servers. Each group-server assignment is stored in the `group_servers` junction table, which already holds many optional config columns (circuit breaker, rate limit, token thresholds, supported models, etc.). The proxy loads this config via `resolve_group_config()` and caches it in Redis as `GroupConfig`. At request time the failover loop iterates servers and applies skip conditions before attempting each one.

The goal is to add a time-of-day restriction to this existing skip-condition chain. Admins need to mark servers as only available in certain hours (e.g., daytime-only regional APIs) so the proxy automatically bypasses them outside that window.

## Goals / Non-Goals

**Goals:**
- Allow admins to configure a daily time window (start, end, timezone) for each group-server assignment
- Skip servers outside their active window during failover, fail-open when config is incomplete or the timezone is unrecognizable
- Support overnight windows (e.g., 22:00-06:00)
- Expose the 3 fields in the admin API (GET and PUT) with all-or-nothing validation
- Display the active hours config and a live "currently inactive" indicator in the frontend

**Non-Goals:**
- Multiple time windows per server
- Day-of-week filtering
- A dedicated API endpoint to query whether a server is currently active
- Changes to proxy logging for the active hours skip
- DST edge-case handling beyond what `chrono-tz` provides automatically

## Decisions

### D1: Three nullable TEXT columns (not a JSONB object or separate table)

All other per-assignment config values in `group_servers` are individual nullable columns. Following that established pattern keeps migrations, SQL queries, serde derivation, and Redis serialization consistent. A separate table would add a join; JSONB would require custom serde. All three fields (start, end, timezone) always travel together, but storing them as three columns is no more complex than a struct and is already the project convention.

### D2: "HH:MM" TEXT format for times (not TIME type or integer minutes)

The admin sends and stores times as strings. Using PostgreSQL `TIME` type would require parsing and formatting on both ends with no benefit since we don't do arithmetic on the stored value — only parse it once at request time with `chrono`. Storing as TEXT keeps the column type trivial and the API contract visible.

### D3: Fail-open on partial config and bad timezone

Consistent with the project's existing fail-open stance on Redis unavailability and circuit breaker errors. Silently skipping the check (treating as 24/7) prevents a misconfiguration from causing all requests to fail. A `warn!` log is emitted so operators can discover the misconfiguration without user-facing breakage.

### D4: All-or-nothing validation in admin API

If only 1 or 2 of the 3 fields are stored, the proxy ignores them (fail-open). Rather than letting the DB accumulate inconsistent partial state, the admin API enforces: either all 3 are provided or all 3 must be null. This mirrors the existing rate limit all-or-nothing pattern (`max_requests` + `rate_window_seconds`).

### D5: `chrono-tz` for IANA timezone parsing

`chrono` is already a dependency. `chrono-tz` is the canonical companion crate for IANA timezone database support and has zero unsafe code. Alternative: ship the IANA tzdata as a file and use a lower-level parser, which is significantly more complex. `chrono-tz` resolves DST transitions automatically.

### D6: Runtime time check (not cached)

The "is it active now?" comparison is a cheap local computation (no I/O). Caching the boolean would require a separate TTL-bounded key and introduce clock skew between the proxy and the cache invalidation edge. The config data (start, end, timezone strings) is cached in GroupConfig/Redis as usual; only the time comparison runs live.

### D7: Active hours is skip condition #11 — placed after rate limit, before request attempt

Ordering: circuit breaker → rate limit → min/max token threshold → per-server model filter → **active hours** → attempt. Active hours is a simple local check with no Redis I/O, so it belongs after the Redis-dependent skip conditions to avoid redundant work when earlier conditions already skip the server.

## Risks / Trade-offs

- **DST ambiguity** → `chrono-tz` handles DST transitions using the IANA tz database compiled into the binary. The active-hours window is always interpreted in the server's configured timezone, including DST. A window that spans a DST boundary may shift by 1 hour on transition night; this is expected and acceptable for the use case (daily operational windows). Mitigation: document in UI hint.
- **Binary size increase** → `chrono-tz` bundles the full IANA tz database (~1 MB compressed). Acceptable given the binary is an internal tool.
- **Stale timezone data** → If an admin enters an IANA name that is valid today but removed in a future `chrono-tz` update, the parse will fail and the server will be treated as 24/7 (fail-open). Mitigation: the frontend validates against the same list it displays.
- **Clock accuracy** → The proxy reads system time at request time. If the host clock drifts, the active hours boundary may be off by the drift amount. No mitigation required for this use case.

## Migration Plan

1. Deploy migration 039: `ALTER TABLE group_servers ADD COLUMN active_hours_start TEXT, ADD COLUMN active_hours_end TEXT, ADD COLUMN active_hours_timezone TEXT;` — fully backward compatible (all NULL).
2. Deploy new backend binary — existing cached `GroupConfig` objects in Redis deserialize correctly because the new fields carry `#[serde(default)]`.
3. Deploy new frontend — immediately allows admins to set active hours on any group-server assignment.
4. No rollback complexity: removing the columns in a future migration is the only rollback step. The proxy skip condition is guarded by `if all 3 fields are Some`, so removing data from the columns restores 24/7 behavior.
