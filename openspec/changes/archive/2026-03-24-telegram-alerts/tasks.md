## 1. Database Migration

- [x] 1.1 Create `viber-router-api/migrations/009_telegram_settings.sql` — table `settings` with columns: `id INT PRIMARY KEY DEFAULT 1`, `telegram_bot_token TEXT`, `telegram_chat_ids TEXT[] NOT NULL DEFAULT '{}'`, `alert_status_codes INT[] NOT NULL DEFAULT '{500,502,503}'`, `alert_cooldown_mins INT NOT NULL DEFAULT 5`; add CHECK constraint `id = 1` to enforce single-row ← (verify: migration runs cleanly, table exists with correct schema and defaults)

## 2. Backend — Settings Model & API

- [x] 2.1 Create `viber-router-api/src/models/settings.rs` — `Settings` struct deriving `FromRow`, `Serialize`, `Deserialize` with all 4 fields; add `pub mod settings;` to `src/models/mod.rs`
- [x] 2.2 Create `viber-router-api/src/routes/admin/settings.rs` — GET `/api/admin/settings` handler: fetch row by id=1, return defaults if not found; PUT handler: upsert via `INSERT ... ON CONFLICT (id) DO UPDATE SET ...`; register routes in `admin/mod.rs` under `/settings`
- [x] 2.3 Add POST `/api/admin/settings/test` handler — validate token and chat_ids not empty (400 if missing), call Telegram `sendMessage` for each chat_id using `state.http_client`, return 200 or 502 ← (verify: all 4 test-alert spec scenarios — success, no chat IDs, no token, Telegram rejects token)
- [x] 2.4 Add GET `/api/admin/settings/telegram-chats` handler — validate token not null (400), call `https://api.telegram.org/bot{token}/getUpdates?limit=100`, deduplicate by `chat.id`, return list with `chat_id`, `first_name`, `username` ← (verify: all 4 chat-discovery spec scenarios — chats found, empty, no token, Telegram error)

## 3. Backend — Telegram Notifier

- [x] 3.1 Create `viber-router-api/src/telegram_notifier.rs` — async fn `maybe_alert(db: &PgPool, redis: &Pool, http_client: &reqwest::Client, server_id: Uuid, server_name: &str, group_name: &str, status_code: u16, latency_ms: i32)`: load settings from DB, skip if token null or status not in alert_status_codes; check Redis key `tg:cooldown:{server_id}:{status_code}` with SET NX EX; send to all chat_ids in parallel; log warn on delivery failure; add `mod telegram_notifier;` to `main.rs`
- [x] 3.2 Update `viber-router-api/src/routes/proxy.rs` — after `emit_log_entry` calls where `request_path == "/v1/messages"` and status is non-200, spawn `tokio::spawn(telegram_notifier::maybe_alert(...))` with cloned db/redis/http_client from state ← (verify: alert fires on /v1/messages errors, does NOT fire on other paths, does NOT block proxy response)

## 4. Frontend — Settings Page

- [x] 4.1 Create `src/pages/SettingsPage.vue` — form with: Bot Token input (text, clearable), Chat IDs list (chips with remove button), "Get Chat IDs from bot" button, Alert Status Codes (chip input for integers), Cooldown minutes (number input), "Test Alert" button, "Save Settings" button; load settings on mount via GET `/api/admin/settings`
- [x] 4.2 Implement "Get Chat IDs from bot" in `SettingsPage.vue` — call GET `/api/admin/settings/telegram-chats`, show loading state, display dialog/list with chat entries (first_name, username, chat_id), allow multi-select, "Add Selected" appends to chat_ids deduplicating; show "No chats found. Send a message to your bot first." when empty; show inline error on API failure
- [x] 4.3 Implement Save and Test in `SettingsPage.vue` — Save calls PUT `/api/admin/settings`, shows success Notify or inline error; Test calls POST `/api/admin/settings/test`, shows success or error Notify ← (verify: all UI spec scenarios — save success, save fail, test success, test fail, navigate to settings)
- [x] 4.4 Add `/settings` route to `src/router/routes.ts` under the main layout children
- [x] 4.5 Add "Settings" nav item to `src/layouts/MainLayout.vue` sidebar with `settings` icon and active state for `/settings` path

## 5. Final Check

- [x] 5.1 Run `just check` — fix all type errors and lint errors in both frontend and backend ← (verify: `just check` exits 0, no clippy warnings, no TypeScript errors)
