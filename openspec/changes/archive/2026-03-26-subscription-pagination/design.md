## Context

The `sub-key-cost-limits` change added a subscriptions system where each sub-key can have multiple subscriptions (active, cancelled, expired, exhausted). The admin UI displays these in an expanded row within the Keys tab of GroupDetailPage.

Currently, `GET /api/admin/groups/:id/keys/:key_id/subscriptions` returns all subscriptions as a flat array. The frontend q-table uses `hide-pagination` with `rowsPerPage: 10`, rendering everything client-side. As subscriptions accumulate over a key's lifetime, this becomes unbounded.

The sub-keys table already implements server-side pagination via `PaginatedResponse<T>` with `page`/`limit` query params — this is the established pattern.

## Goals / Non-Goals

**Goals:**
- Server-side pagination for the key subscriptions list endpoint
- Frontend q-table with server-side pagination controls matching the sub-keys table pattern
- Maintain current page position after add/cancel operations

**Non-Goals:**
- Search/filter on subscriptions (not needed — subscriptions are scoped to a single key)
- Changing the subscription data model or cost calculation logic
- Modifying Redis cache strategy (cache is for proxy resolution, not admin list)

## Decisions

**1. Follow existing `group_keys` pagination pattern exactly**

The `list_keys` handler in `group_keys.rs` uses `ListParams { page, limit, search }`, COUNT query, LIMIT/OFFSET, and returns `PaginatedResponse<T>`. The subscriptions handler will use the same pattern minus `search` (not needed for per-key scoped data).

Alternative: Cursor-based pagination — rejected because all other admin endpoints use offset-based, and the dataset per key is small enough that offset works fine.

**2. Default 10 rows per page**

Matches the current `rowsPerPage: 10` setting. The subscriptions table is nested inside an expanded row, so keeping it compact is important.

**3. Per-key pagination state**

Frontend needs to track pagination state per key since multiple keys can be expanded simultaneously. Store as `Record<string, { data, total, page, rowsPerPage }>` keyed by key ID.

**4. Cost calculation stays in the loop**

The current handler iterates subscriptions and calls `get_total_cost` per sub. With pagination (max 10 per page), this is at most 10 cost lookups per request — acceptable. No need to optimize.

## Risks / Trade-offs

**[Pagination + cost calculation]** Each page load still does N cost lookups (one per subscription). With limit=10 this is fine, but if someone sets a very high limit it could be slow. → Mitigation: Clamp limit to max 100 (same as group_keys).

**[COUNT query overhead]** An extra COUNT query per page load. → Mitigation: The `(group_key_id, status)` index covers this query efficiently. Negligible overhead.
