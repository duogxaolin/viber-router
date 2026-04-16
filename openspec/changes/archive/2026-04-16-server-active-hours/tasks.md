## 1. Database Migration

- [x] 1.1 Create migration file `viber-router-api/migrations/039_group_servers_active_hours.sql` with `ALTER TABLE group_servers ADD COLUMN active_hours_start TEXT, ADD COLUMN active_hours_end TEXT, ADD COLUMN active_hours_timezone TEXT`
- [x] 1.2 Run `sqlx migrate run` against the development database to verify the migration applies cleanly ← (verify: migration applies without error, all three columns appear in `\d group_servers`, existing rows have NULL values for new columns)

## 2. Backend Dependency

- [x] 2.1 Add `chrono-tz` to `viber-router-api/Cargo.toml` under `[dependencies]` with a version compatible with the existing `chrono` dependency
- [x] 2.2 Run `cargo check` in `viber-router-api/` to confirm the crate resolves and compiles

## 3. Backend Models

- [x] 3.1 Add `active_hours_start: Option<String>`, `active_hours_end: Option<String>`, `active_hours_timezone: Option<String>` to the `GroupServer` struct with `#[serde(default)]` on each field
- [x] 3.2 Add the same three fields with `#[serde(default)]` to `GroupServerDetail`
- [x] 3.3 Add the same three fields with `#[serde(default)]` to `AdminGroupServerDetail`
- [x] 3.4 Run `cargo check` to confirm model changes compile without errors ← (verify: no compile errors, serde default attributes present so that deserialization of old Redis data missing these fields succeeds)

## 4. Backend Admin API

- [x] 4.1 Add `active_hours_start: Option<Option<String>>`, `active_hours_end: Option<Option<String>>`, `active_hours_timezone: Option<Option<String>>` to the `UpdateAssignment` request struct in `group_servers.rs` (double-Option pattern: outer = was field provided, inner = value)
- [x] 4.2 Add all-or-nothing validation in the `update_assignment` handler: if exactly 1 or 2 of the three inner values are Some, return HTTP 400 with an appropriate error message
- [x] 4.3 Add `HH:MM` format validation for start and end fields when non-null (regex or manual parse: hours 00-23, minutes 00-59)
- [x] 4.4 Add IANA timezone validation for the timezone field when non-null: attempt to parse with `chrono_tz::Tz::from_str()` and return HTTP 400 if it fails
- [x] 4.5 Add the three fields to the `UPDATE group_servers SET ...` query in the handler, applying them only when their outer Option is Some
- [x] 4.6 Confirm `invalidate_group_all_keys()` is already called after the update (no change needed — verify existing code path covers it)
- [x] 4.7 Run `cargo clippy -- -D warnings` and fix any warnings ← (verify: all validation scenarios from specs/group-server-assignment/spec.md pass; partial config, bad format, and bad timezone each return HTTP 400; valid update clears Redis cache)

## 5. Backend Proxy Failover

- [x] 5.1 Add the three active hours column names to the `SELECT` column list in `resolve_group_config()` in `proxy.rs` (they are already in the model structs; the query must explicitly name them)
- [x] 5.2 Implement an `is_server_active_now(server: &GroupServerDetail) -> bool` helper function (or inline logic) that: checks all three fields are Some; parses the timezone with `chrono_tz::Tz::from_str()`; gets current UTC time, converts to the target timezone; extracts HH:MM as (hour, minute); compares against parsed start and end using the overnight-window logic (if start <= end: active when start <= now <= end; if start > end: active when now >= start OR now <= end); returns true (fail-open) if any field is None or timezone parse fails, logging a warning for parse failures
- [x] 5.3 Insert the active hours skip condition in the failover loop after the existing per-server model filter check and before the request attempt: `if !is_server_active_now(&server) { continue; }`
- [x] 5.4 Run `cargo clippy -- -D warnings` and fix any warnings
- [x] 5.5 Run `cargo test` to ensure existing tests pass ← (verify: proxy compiles with new skip condition; all scenarios from specs/proxy-engine/spec.md are exercised — especially overnight window, fail-open for incomplete config, and fail-open for bad timezone with warn log)

## 6. Frontend — Server Card Badge

- [x] 6.1 In `GroupDetailPage.vue`, locate the server card rendering section and add a `q-badge` (or equivalent chip/tag) that displays `{active_hours_start}-{active_hours_end} ({active_hours_timezone})` when all three fields are non-null on the server assignment
- [x] 6.2 Ensure the badge is not rendered when any of the three fields is null ← (verify: badge appears on a server card with active hours set; badge absent when fields are null)

## 7. Frontend — Edit Server Dialog Active Hours Section

- [x] 7.1 Add a curated list of common IANA timezone strings as a constant in the component or a separate data file (include at minimum: UTC, US/Eastern, US/Central, US/Mountain, US/Pacific, Europe/London, Europe/Paris, Europe/Berlin, Asia/Tokyo, Asia/Shanghai, Asia/Ho_Chi_Minh, Asia/Bangkok, Asia/Singapore, Australia/Sydney, America/Sao_Paulo)
- [x] 7.2 Add a reactive form state for `activeHoursStart`, `activeHoursEnd`, `activeHoursTimezone` in the edit dialog's data/setup, initialized from the server assignment when the dialog opens
- [x] 7.3 Add the "Active Hours" section to the Edit Server Dialog with: a filterable `q-select` bound to `activeHoursTimezone` using the IANA list from 7.1, a `q-input` with mask `##:##` bound to `activeHoursStart`, a `q-input` with mask `##:##` bound to `activeHoursEnd`, hint text "Leave empty for 24/7. Overnight windows supported (e.g., 22:00-06:00).", and a "Clear" button that sets all three to null/empty
- [x] 7.4 Implement all-or-nothing validation: compute a `activeHoursValid` computed property that returns true when all three are filled OR all three are empty; return false (with error text) when only 1 or 2 are filled
- [x] 7.5 Wire the validation into the save button disabled state alongside other field validations
- [x] 7.6 Include the three fields in the PUT request payload when saving: send null when empty, send the string value when filled ← (verify: dialog pre-populates from existing assignment; clear button resets all three; partial fill blocks save; valid save sends correct payload and updates the server card badge)

## 8. Final Checks

- [x] 8.1 Run `just check` (type-check + lint for both frontend and backend) and fix any errors reported ← (verify: `just check` exits 0 with no errors or warnings)
