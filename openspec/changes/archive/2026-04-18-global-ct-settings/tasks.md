## 1. Database Migration

- [x] 1.1 Create `viber-router-api/migrations/041_add_ct_settings.sql` adding global token counting settings columns

## 2. Backend Models

- [x] 2.1 Add global CT settings fields to settings model in `viber-router-api/src/models/settings.rs`

## 3. Admin Settings Handler

- [x] 3.1 Expose global CT settings via admin API in `viber-router-api/src/routes/admin/settings.rs`

## 4. Proxy Engine

- [x] 4.1 Implement global CT override logic in `viber-router-api/src/routes/proxy.rs` — when always-estimate is enabled, proxy handles count_tokens requests directly instead of forwarding to per-group servers

## 5. Telegram Notifier

- [x] 5.1 Update `viber-router-api/src/telegram_notifier.rs` to account for global CT settings

## 6. Frontend UI

- [x] 6.1 Add global CT settings controls (always-estimate toggle, Anthropic base URL, API key) to `src/pages/SettingsPage.vue`

## 7. Final Check

- [x] 7.1 Run `just check` — passed with 0 CRITICAL, 0 WARNING, build clean
