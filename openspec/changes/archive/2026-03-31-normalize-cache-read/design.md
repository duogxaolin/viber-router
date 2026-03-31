## Context

Viber Router proxies LLM API requests and tracks token usage and costs. Anthropic Claude splits input tokens into three categories: `input_tokens` (non-cached), `cache_creation_input_tokens` (written to cache, 1.25x input price), and `cache_read_input_tokens` (read from cache, 0.1x input price). Other upstreams (OpenAI, etc.) report all tokens as `input_tokens`.

This asymmetry causes two problems:
1. Cost comparisons across servers are unfair — a Claude server with heavy cache hits looks cheaper in raw token counts but the cost calculation is correct; however, the public usage page shows inflated "input" for non-Claude servers and fragmented columns for Claude.
2. Users on the public usage page see confusing Cache W / Cache R columns that are always zero for non-Claude servers.

The `normalize_cache_read` flag is a per-server-assignment setting (on `group_servers`) that, when enabled, prices cache-read tokens at the input rate and collapses cache columns in the public view.

## Goals / Non-Goals

**Goals:**
- Add `normalize_cache_read` boolean to `group_servers` with default `false`
- Modify `calculate_cost()` to accept and apply the flag for real-time cost tracking
- Thread the flag through proxy.rs (both streaming and non-streaming paths)
- Public usage API returns `effective_input_tokens` instead of separate cache columns
- Public usage page shows a single Input column (effective) and hides Cache W / Cache R
- Admin Cost Rates modal exposes a toggle for the flag
- No data backfill — flag only affects new requests

**Non-Goals:**
- Changing the admin token usage stats display (raw columns stay)
- Retroactively recalculating `cost_usd` for existing rows
- Changing how `cache_creation_input_tokens` is priced (always 1.25x input)
- Any changes to how tokens are extracted from upstream responses

## Decisions

### D1: Flag lives on `group_servers`, not on `servers` or `groups`

The flag is a billing policy decision per group-server assignment, not a property of the server itself. A server could be assigned to multiple groups with different normalization policies. This matches the existing pattern for `rate_input`, `rate_output`, etc.

Alternatives considered:
- Per-server flag: rejected — same server could be used in different billing contexts
- Per-group flag: rejected — a group might mix Claude and non-Claude servers

### D2: `calculate_cost()` receives a `normalize_cache_read: bool` parameter

The function already takes all rate multipliers as parameters. Adding a boolean keeps the signature consistent and avoids threading `GroupServerDetail` into `subscription.rs`. The caller (proxy.rs) reads the flag from the server config and passes it.

Alternatives considered:
- Passing `GroupServerDetail` directly: rejected — creates coupling between subscription.rs and the model layer
- Replacing `cache_read_1m_usd` with `input_1m_usd` at the call site: rejected — obscures intent

### D3: Public usage API changes response shape (replaces cache columns with `effective_input_tokens`)

The public endpoint is consumed only by `PublicUsagePage.vue`. Changing the response shape is safe. Returning both old and new fields would add noise. The admin endpoint is separate and unchanged.

Alternatives considered:
- Adding `effective_input_tokens` alongside existing columns: rejected — frontend would need to decide which to show, and old columns would be misleading
- Computing effective_input in the frontend: rejected — requires sending raw cache columns over the wire unnecessarily

### D4: No migration for existing `cost_usd` data

The flag is a forward-only policy. Retroactive recalculation would require knowing which server handled each historical request and whether the flag was set at that time — complexity not worth the benefit for an internal admin tool.

### D5: `GroupServerDetail` (proxy cache struct) includes `normalize_cache_read`

The flag must be available at request time without a DB lookup. `GroupServerDetail` is already cached in Redis via `GroupConfig`. Adding the field follows the same pattern as `rate_input`, `max_requests`, etc. Cache invalidation already happens on assignment updates.

## Risks / Trade-offs

- [Flag default is `false`] → Existing behavior is preserved for all current assignments. No surprise cost changes.
- [Public API shape change] → Any external consumer of the public usage endpoint would break. Acceptable: this is an internal tool and the endpoint is authenticated by subscription key.
- [Redis cache holds old `GroupServerDetail` without the new field] → After deploy, cached configs won't have `normalize_cache_read`. Serde `#[serde(default)]` on the field ensures deserialization succeeds and defaults to `false`. Behavior is correct (conservative) until cache expires or is invalidated.
- [Admin usage SQL unchanged] → The admin view still shows raw `cache_read` cost, which may differ from what was actually charged if the flag was set. This is intentional — admin sees raw data.

## Migration Plan

1. Deploy DB migration (`ALTER TABLE group_servers ADD COLUMN normalize_cache_read BOOLEAN NOT NULL DEFAULT false`)
2. Deploy backend — new field defaults to `false`, existing Redis cache deserializes safely via `#[serde(default)]`
3. Deploy frontend — public usage page shows new column layout; admin can toggle the flag per server assignment
4. No rollback complexity — removing the column would require a migration, but the flag defaults to `false` so disabling the feature in code is sufficient for rollback

## Open Questions

None — all decisions are resolved per the feature description.
