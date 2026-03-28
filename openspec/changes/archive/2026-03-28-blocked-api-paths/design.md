## Context

The proxy (`proxy_handler` in `routes/proxy.rs`) currently forwards all incoming API paths to upstream servers. The `settings` table (single row, id=1) stores global configuration (currently Telegram alert settings). The Settings UI (`SettingsPage.vue`) provides admin management of these settings.

Admin needs the ability to globally block specific API paths (e.g., `/v1/completions`) so the proxy returns 404 immediately — before any authentication or routing logic runs.

## Goals / Non-Goals

**Goals:**
- Allow admins to manage a list of blocked API paths via the Settings UI
- Block matching requests at the earliest point in proxy_handler (before API key extraction)
- Return Anthropic-style 404 JSON for blocked paths
- Cache blocked paths in Redis for fast proxy-time lookup

**Non-Goals:**
- Per-group or per-key path blocking (this is global only)
- Wildcard or regex pattern matching (exact path match only)
- Blocking based on HTTP method (all methods blocked for a matched path)
- Logging or analytics for blocked requests

## Decisions

### Decision 1: Check position in proxy flow
**Choice**: First check in `proxy_handler`, before API key extraction.
**Rationale**: Blocked paths should appear as if they don't exist (404). Checking before auth means no API key is needed, and the response is identical regardless of authentication state.
**Alternative**: After auth — rejected because it would require a valid API key to discover a path is blocked, which is inconsistent with 404 semantics.

### Decision 2: Redis cache for blocked paths
**Choice**: Dedicated Redis key `settings:blocked_paths` storing a JSON array of path strings. Loaded once per proxy request via a single `GET` command. Invalidated when admin updates settings.
**Rationale**: The group config cache pattern already exists (`group:config:{key}`). A separate key avoids coupling blocked paths to per-group config. One Redis GET per request is negligible overhead.
**Alternative**: In-memory cache with TTL — rejected because it adds complexity (refresh logic, stale window) and the existing codebase uses Redis for all caching.

### Decision 3: Path matching strategy
**Choice**: Exact match on the URI path component. Query strings are naturally excluded since `original_uri.path()` returns only the path portion.
**Rationale**: Simple, predictable, no regex overhead. Admin blocks `/v1/completions` and it matches exactly `/v1/completions` regardless of query parameters.

### Decision 4: Fallback when Redis is unavailable
**Choice**: If Redis GET fails, allow the request through (fail-open).
**Rationale**: Blocked paths are a convenience feature, not a security boundary. Failing open ensures proxy availability is not degraded by Redis outages. This is consistent with how the group config cache handles Redis failures (falls back to DB).

### Decision 5: Settings model extension
**Choice**: Add `blocked_paths: Vec<String>` to the existing `Settings` struct and `settings` table. Reuse the existing GET/PUT settings API.
**Rationale**: Blocked paths are a global setting, same as Telegram config. No need for a separate table or API endpoint.

## Risks / Trade-offs

- **Redis GET on every request** → Negligible latency (~0.1ms). Acceptable for the simplicity it provides over in-memory caching.
- **Fail-open on Redis failure** → Blocked paths temporarily not enforced during Redis outage. Acceptable since this is not a security feature.
- **No cache TTL** → Blocked paths cache persists until explicitly invalidated. If invalidation fails, stale data remains. Mitigation: admin can re-save settings to re-trigger invalidation.
