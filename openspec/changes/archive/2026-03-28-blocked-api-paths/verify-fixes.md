## [2026-03-28] Round 1 (from spx-apply auto-verify)

### spx-verifier
- Fixed: Unconditional `invalidate_blocked_paths` call in `put_settings` — now only invalidates when `blocked_paths` is present in the request body (`settings.rs`)
- Fixed: `get_blocked_paths` now returns `Result<Option<Vec<String>>, ()>` to distinguish Redis failure from cache miss — proxy handler uses `Err(())` arm for true fail-open on Redis failure (`cache.rs`, `proxy.rs`)

### spx-uiux-verifier
- Fixed: Added `remove-aria-label` prop to blocked path chips for screen reader accessibility (`SettingsPage.vue`)
- Fixed: Added `role="group" aria-label="Blocked API Paths"` to chip container (`SettingsPage.vue`)
- Fixed: Added `saveError` display inside the Blocked API Paths card above its Save button (`SettingsPage.vue`)
- Fixed: `addBlockedPath` now shows a warning notification when a duplicate path is entered (`SettingsPage.vue`)
