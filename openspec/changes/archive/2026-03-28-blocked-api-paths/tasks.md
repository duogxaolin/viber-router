## 1. Database

- [x] 1.1 Create migration `027_add_blocked_paths_to_settings.sql` — add `blocked_paths TEXT[] NOT NULL DEFAULT '{}'` column to `settings` table

## 2. Backend Model & Cache

- [x] 2.1 Add `blocked_paths: Vec<String>` to `Settings` struct in `src/models/settings.rs` (with `#[serde(default)]`)
- [x] 2.2 Add `get_blocked_paths` and `set_blocked_paths` functions in `src/cache.rs` — Redis key `settings:blocked_paths`, JSON array format
- [x] 2.3 Add `invalidate_blocked_paths` function in `src/cache.rs` — deletes Redis key `settings:blocked_paths` ← (verify: cache functions compile, key name is `settings:blocked_paths`)

## 3. Backend Settings API

- [x] 3.1 Update `default_settings()` in `src/routes/admin/settings.rs` to include `blocked_paths: vec![]`
- [x] 3.2 Update `get_settings` query to SELECT `blocked_paths` column
- [x] 3.3 Add `blocked_paths: Option<Vec<String>>` to `UpdateSettings` struct
- [x] 3.4 Update `put_settings` to merge and persist `blocked_paths`, and call `invalidate_blocked_paths` after successful upsert ← (verify: GET returns blocked_paths, PUT saves and invalidates Redis cache)

## 4. Proxy Blocked Path Check

- [x] 4.1 Add blocked path check as the first operation in `proxy_handler` — before API key extraction. Load blocked paths from Redis (via `get_blocked_paths`), on cache miss query DB and populate cache. If path matches, return 404 with `{"type":"error","error":{"type":"not_found_error","message":"Not found"}}`. On Redis failure, fail-open (allow request through). ← (verify: blocked path returns 404 before auth, non-blocked path proceeds normally, Redis failure allows request through)

## 5. Frontend Settings UI

- [x] 5.1 Add `blocked_paths: string[]` to the `Settings` interface in `SettingsPage.vue`
- [x] 5.2 Add "Blocked API Paths" section to the template — chip list with removable chips, text input to add new paths (Enter key or Add button), empty state text "No blocked paths"
- [x] 5.3 Wire `blocked_paths` into the save payload sent to PUT `/api/admin/settings` ← (verify: paths display as chips, add/remove works, save persists to server)

## 6. Verify

- [x] 6.1 Run `just check` — ensure no type errors or lint warnings in both frontend and backend
