## Context

The proxy already has three async buffer systems (log_buffer, ttft_buffer, usage_buffer) that follow the same pattern: mpsc channel → background flush task → batch INSERT into a partitioned PostgreSQL table. The partition manager handles monthly partitions and retention cleanup for all three tables.

The proxy handler iterates through servers in priority order (failover chain). For each server attempt, it records the status code and latency. Currently this data is only captured in `proxy_logs.failover_chain` (JSONB), but only for error/failover requests — successful single-server requests are not logged to proxy_logs.

The GroupDetailPage shows servers in a list with priority, circuit breaker status, and rate limit badges. The PublicUsagePage shows sub-key usage data, subscriptions, and TTFT charts.

## Goals / Non-Goals

**Goals:**
- Record every server attempt in the proxy chain for uptime analysis
- Provide per-server uptime bars (90 × 30-minute buckets) in the admin group detail page
- Provide chain-level uptime status + bars in the public usage page
- Follow existing buffer/partition patterns exactly for consistency

**Non-Goals:**
- Active health checks (pinging servers independently of traffic)
- Historical uptime beyond LOG_RETENTION_DAYS
- Per-model or per-key uptime breakdown
- Alerting on uptime degradation (can be added later)

## Decisions

### D1: New `uptime_checks` partitioned table with lightweight records

Each server attempt in the proxy chain emits one record: (group_id, server_id, status_code, latency_ms, request_id, created_at). The `request_id` UUID is generated once per proxy request and shared across all attempts in the same chain.

**Rationale**: Querying JSONB arrays in proxy_logs.failover_chain would be slow and proxy_logs doesn't capture successful single-server requests. A dedicated table with proper indexes enables fast time-bucket aggregation.

**Alternatives considered**:
- Aggregate from existing proxy_logs + token_usage_logs: fragile coupling, JSONB unnest performance issues, missing data for successful first-server requests
- Active health checks: doesn't reflect real traffic patterns, adds external dependencies

### D2: Async mpsc buffer (uptime_buffer.rs)

Same pattern as ttft_buffer: bounded mpsc channel (10,000 capacity), background flush task, batch INSERT every 5 seconds or 100 records. Overflow drops entries with tracing::warn.

**Rationale**: Proven pattern in this codebase. Zero proxy latency impact.

### D3: Time-bucket aggregation in SQL

The API endpoints compute 30-minute buckets using `floor(extract(epoch from created_at) / 1800) * 1800` and aggregate success rates per bucket. 90 buckets covers ~45 hours.

**Rationale**: Simple SQL aggregation with proper indexes. No materialized views or pre-computation needed — the data volume per group is manageable (even at 1000 req/30min, 90 buckets × N servers is a small result set).

### D4: Chain success derived from request_id grouping

For the public page, chain-level success is determined by: "does any attempt with this request_id have a 2xx status code?" This is computed via `COUNT(DISTINCT request_id) FILTER (WHERE status_code BETWEEN 200 AND 299)`.

**Rationale**: The request_id links all attempts in a single proxy request. If any server succeeded, the chain succeeded — matching the proxy's actual failover behavior.

### D5: Shared UptimeBars component

A single Vue component renders the status bars for both admin (per-server) and public (chain-level) views. It accepts an array of `{ bucket, total, success }` objects and renders colored bars.

**Rationale**: Same visual pattern in both pages. DRY.

## Risks / Trade-offs

- **Storage growth**: Every proxy request generates 1-N uptime_check records (one per server attempted). At high traffic, this adds significant rows. → Mitigated by partition cleanup via existing LOG_RETENTION_DAYS mechanism.
- **Query performance on large datasets**: Aggregating 45 hours of data per request. → Mitigated by composite index on (group_id, server_id, created_at). The query scans a narrow time range.
- **Buffer overflow under extreme load**: If proxy handles >10,000 concurrent requests with multiple server attempts each, the buffer may overflow. → Acceptable: entries are dropped with a warning, same as other buffers. Uptime data is best-effort.
