## Why

Anthropic Claude reports input tokens across three separate fields (`input_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`), while other upstreams (OpenAI, etc.) report everything as `input_tokens`. This makes cross-server usage comparisons unfair and inflates apparent "input" costs for Claude users who benefit from prompt caching. A per-server flag lets admins normalize cache-read tokens to input pricing, making usage statistics comparable across providers.

## What Changes

- New `normalize_cache_read` boolean column on `group_servers` (default `false`)
- `calculate_cost()` gains a `normalize_cache_read` flag: when true, cache-read tokens are priced at `input_1m_usd × rate_input` instead of `cache_read_1m_usd × rate_cache_read`
- Proxy (both streaming and non-streaming paths) reads the flag from `GroupServerDetail` and passes it to `calculate_cost()`
- Public usage API returns `effective_input_tokens = input + cache_creation + cache_read` instead of separate cache columns
- Public usage page shows a single Input column (effective) and hides Cache W / Cache R columns
- Admin Cost Rates modal gains a "Normalize Cache Read" toggle
- Admin usage stats page is unchanged (raw columns kept)

## Capabilities

### New Capabilities

- `normalize-cache-read`: Per-server flag that controls whether cache-read tokens are billed at input price and whether the public usage page collapses cache columns into a single effective-input column

### Modified Capabilities

- `server-cost-rate`: `calculate_cost()` signature and behavior changes — adds `normalize_cache_read` parameter that alters the cache-read cost formula
- `public-usage-api`: Response shape changes — replaces `total_cache_creation_tokens` / `total_cache_read_tokens` with `effective_input_tokens`
- `public-usage-page`: Table columns change — Input column shows effective tokens, Cache W and Cache R columns removed

## Impact

- **DB migration**: `ALTER TABLE group_servers ADD COLUMN normalize_cache_read BOOLEAN NOT NULL DEFAULT false`
- **Rust**: `GroupServer`, `GroupServerDetail`, `AdminGroupServerDetail`, `UpdateAssignment` structs; `calculate_cost()` in `subscription.rs`; proxy.rs non-streaming and streaming paths; `token_usage.rs` admin SQL; `public/usage.rs` query and response struct
- **Frontend**: `GroupServerDetail` interface in `stores/groups.ts`; Cost Rates modal in `GroupDetailPage.vue`; `ModelUsage` interface and table in `PublicUsagePage.vue`
- **No breaking changes to admin token usage API** — raw columns unchanged
- **No data backfill** — flag only affects new requests; existing `cost_usd` rows are untouched
