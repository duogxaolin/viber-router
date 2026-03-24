## Context

The proxy currently logs all upstream errors to PostgreSQL but provides no real-time alerting. Operators must manually check the Logs page to discover issues. The system already uses Redis (deadpool-redis) for group config caching and `reqwest` for upstream HTTP calls — both are available for the Telegram integration without new dependencies.

Current error flow: `proxy_handler` → `emit_log_entry` → `log_tx` channel → PostgreSQL. Alerts will hook into `proxy_handler` after the log entry is emitted, as a separate fire-and-forget path.

## Goals / Non-Goals

**Goals:**
- Real-time Telegram alerts when upstream servers return configured error codes on `POST /v1/messages`
- Configurable via Admin UI: bot token, multiple chat IDs, alert status codes, cooldown per server+code
- Zero impact on proxy latency (fully async, fire-and-forget)
- Cooldown suppresses duplicate alerts for the same `server_id + status_code` pair

**Non-Goals:**
- Alerting on non-`/v1/messages` paths
- Per-group alert configuration (global settings only)
- Alert history or delivery receipts
- Webhook-based Telegram updates (polling only via `getUpdates`)

## Decisions

### D1: Settings stored in PostgreSQL, not `.env`

Settings are stored in a single-row `settings` table (upsert by `id = 1`). This allows live reconfiguration via Admin UI without server restart.

Alternative: `.env` vars — rejected because it requires restart and cannot be edited via UI.

### D2: Settings loaded fresh from DB on each alert (no in-memory cache)

The notifier reads settings from DB at alert time rather than caching in `AppState`. Alert frequency is low (errors are infrequent), so the DB read overhead is negligible. This avoids stale config issues when settings are updated via UI.

Alternative: Cache in `AppState` with invalidation — rejected as over-engineering for low-frequency reads.

### D3: Cooldown via Redis key with TTL

Key: `tg:cooldown:{server_id}:{status_code}`, TTL = `alert_cooldown_mins * 60` seconds. Using Redis `SET NX EX` (set if not exists with expiry) as an atomic check-and-set.

Alternative: In-memory `HashMap<(Uuid, u16), Instant>` — rejected because it resets on restart and doesn't work in multi-instance deployments.

### D4: Redis cooldown failure → fail open (still send alert)

If Redis is unavailable when checking cooldown, the notifier proceeds to send the alert. This avoids silent alert suppression during Redis outages.

### D5: Telegram HTTP calls via existing `reqwest::Client` in `AppState`

Reuse the existing `http_client` in `AppState` for Telegram API calls. No new HTTP client needed.

### D6: `chat_ids` stored as `TEXT[]` in PostgreSQL

Multiple chat IDs stored as a PostgreSQL text array. Simple, no join table needed for a global settings row.

### D7: `getUpdates` called with `limit=100`, no offset tracking

The "Get Chat IDs" feature calls `getUpdates?limit=100` without tracking offset. This is intentional — the feature is a discovery helper, not a full update consumer. Users are expected to have recently messaged the bot before using this feature.

### D8: Alert message format (Markdown, sent via `sendMessage` with `parse_mode=Markdown`)

```
🚨 *Upstream Error*
*Server:* claude-prod
*Group:* my-group
*Status:* 500
*Latency:* 1234ms
*Time:* 2026-03-24 10:30:00 UTC
```

Sent to all configured `chat_ids` in parallel (tokio::join_all).

## Risks / Trade-offs

- **[Telegram API rate limits]** → Telegram limits bots to ~30 messages/second. With cooldown enabled this is not a concern in practice. If cooldown is set to 0 and many servers fail simultaneously, rate limiting could occur. Mitigation: document that cooldown ≥ 1 minute is recommended.
- **[getUpdates consumes update queue]** → Calling `getUpdates` without offset acknowledgment means the same updates appear on repeated calls until Telegram's server-side TTL expires (~24h). This is acceptable for a discovery helper.
- **[Single-row settings table]** → Upsert by `id = 1` is a simple pattern but unconventional. Mitigation: clearly documented in migration comment.

## Migration Plan

1. Run migration `009_telegram_settings.sql` — creates `settings` table (additive, no data loss)
2. Deploy backend — new routes and notifier module are additive
3. Deploy frontend — new Settings page is additive
4. No rollback concerns — all changes are additive
