## Why

When upstream servers return errors, operators have no real-time visibility — they must manually check the logs page. This change adds Telegram alert notifications so operators are immediately notified when upstream servers return configured error status codes on `POST /v1/messages`.

## What Changes

- New `settings` database table storing Telegram bot configuration (token, chat IDs, alert status codes, cooldown)
- New backend module `telegram_notifier` that fires alerts asynchronously (fire-and-forget, no proxy latency impact)
- Cooldown mechanism using Redis to suppress duplicate alerts for the same server + status code within a configurable window
- New admin API endpoints: GET/PUT `/api/admin/settings`, POST `/api/admin/settings/test`, GET `/api/admin/settings/telegram-chats`
- New `/settings` page in the Admin UI with form to configure bot token, chat IDs (multi), alert status codes, and cooldown minutes
- "Get Chat IDs from bot" feature that calls Telegram `getUpdates` and presents discovered chats for selection

## Capabilities

### New Capabilities

- `telegram-alert-settings`: CRUD API and UI for storing and managing Telegram bot configuration (token, chat IDs, alert status codes, cooldown)
- `telegram-alert-delivery`: Async alert delivery to Telegram when upstream errors occur on `POST /v1/messages`, with Redis-based cooldown per `server_id + status_code`
- `telegram-chat-discovery`: Fetch and display chats that have messaged the configured bot via Telegram `getUpdates` API

### Modified Capabilities

## Impact

- **Backend**: New migration `009_telegram_settings.sql`, new `src/telegram_notifier.rs` module, new `src/routes/admin/settings.rs`, updated `proxy.rs` to call notifier, updated `AppState` to carry settings cache or DB reference
- **Frontend**: New `src/pages/SettingsPage.vue`, updated `src/router/routes.ts`, updated `src/layouts/MainLayout.vue` (sidebar nav item)
- **Dependencies**: No new Rust crates needed (`reqwest` already present for HTTP calls to Telegram API)
- **Database**: New `settings` table (single-row, upsert pattern)
- **Redis**: New key namespace `tg:cooldown:{server_id}:{status_code}` with TTL
