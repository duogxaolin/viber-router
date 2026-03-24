## [2026-03-24] Round 1 (from spx-apply auto-verify)

### spx-verifier
- Fixed: `check_and_set_cooldown` in `viber-router-api/src/telegram_notifier.rs` — changed return type from `Result<bool, deadpool_redis::PoolError>` to `Result<bool, Box<dyn std::error::Error + Send + Sync>>` and used `?` on `query_async` instead of `.unwrap_or(None)`. This ensures Redis command-level errors (not just pool acquisition errors) propagate to the `Err(e)` branch in `maybe_alert`, which correctly logs a warning and fails open (sends the alert). Previously, command errors were silently swallowed as `None`, causing fail-closed behavior that violated the spec scenario "Redis unavailable during cooldown check — fail open" and Design.md D4.
