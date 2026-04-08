## 1. Database

- [x] 1.1 Create migration file `viber-router-api/migrations/036_add_min_input_tokens_to_group_servers.sql` with `ALTER TABLE group_servers ADD COLUMN min_input_tokens INTEGER NULL;`
- [ ] 1.2 Verify migration applies cleanly with `sqlx migrate run` against a local database ← (verify: column exists in group_servers, existing rows have NULL, no migration errors)

## 2. Backend Models

- [x] 2.1 Add `min_input_tokens: Option<i32>` to `GroupServer` struct in `viber-router-api/src/models/group_server.rs` after the `max_input_tokens` field
- [x] 2.2 Add `min_input_tokens: Option<i32>` to `GroupServerDetail` struct after `max_input_tokens`
- [x] 2.3 Add `min_input_tokens: Option<i32>` to `AdminGroupServerDetail` struct after `max_input_tokens`
- [x] 2.4 Add `min_input_tokens: Option<i32>` to `AssignServer` struct after `max_input_tokens`
- [x] 2.5 Add `min_input_tokens: Option<Option<i32>>` to `UpdateAssignment` struct after `max_input_tokens` ← (verify: double-Option matches max_input_tokens pattern; `cargo check` passes)

## 3. Backend Admin API

- [x] 3.1 In `assign_server` handler (`viber-router-api/src/routes/admin/group_servers.rs`): add `gs.min_input_tokens` to the INSERT column list and bind `payload.min_input_tokens` to the query
- [x] 3.2 In `update_assignment` handler: add the double-optional unwrap for `min_input_tokens` matching the existing `max_input_tokens` pattern (derive `update_min_input_tokens: bool` and `min_input_tokens_val: Option<i32>`)
- [x] 3.3 In the UPDATE SQL query: add `min_input_tokens = CASE WHEN $N THEN $N+1 ELSE min_input_tokens END` with correct sequential parameter indices
- [x] 3.4 Bind `(update_min_input_tokens, min_input_tokens_val)` to the query builder after the `max_input_tokens` binds ← (verify: `cargo clippy -- -D warnings` passes; INSERT and UPDATE round-trip min_input_tokens correctly)

## 4. Backend Proxy

- [x] 4.1 In `viber-router-api/src/routes/proxy.rs`, add `gs.min_input_tokens` to the SELECT column list in the proxy cache loading query (after `gs.max_input_tokens`)
- [x] 4.2 Add `min_input_tokens: Option<i32>` to the server cache struct that maps the SELECT result
- [x] 4.3 After the existing `max_input_tokens` gate in the routing waterfall, add the `min_input_tokens` gate:
  ```rust
  if let Some(limit) = server.min_input_tokens
      && let Some(est) = estimated_tokens
      && est < limit as usize
  {
      continue;
  }
  ```
- [ ] 4.4 Run `cargo clippy -- -D warnings` and confirm no warnings ← (verify: proxy skips server when est < min, does not skip when est >= min or min is NULL, fail-open when estimation absent)

## 5. Frontend Store

- [x] 5.1 Add `min_input_tokens: number | null` to the `GroupServerDetail` interface in `src/stores/groups.ts`
- [x] 5.2 Add `min_input_tokens?: number | null` to the `updateAssignment` input type in `src/stores/groups.ts` ← (verify: `bun run lint` passes; TypeScript strict mode satisfied)

## 6. Frontend UI

- [x] 6.1 In `src/pages/GroupDetailPage.vue`, add `min_input_tokens: null as number | null` to the `editServerTokenForm` ref initializer
- [x] 6.2 In `doOpenEditServer`, populate `min_input_tokens: s.min_input_tokens` when opening the edit dialog
- [x] 6.3 In `onSaveEditServer`, pass `min_input_tokens: editServerTokenForm.value.min_input_tokens` to `updateAssignment`
- [x] 6.4 Add the `q-input` for Min Input Tokens below the existing `max_input_tokens` input (label, type number, min=1, outlined, dense, clearable, `@clear` sets to null)
- [x] 6.5 Add the `q-badge` for `min_input_tokens` in the server list below the `max_input_tokens` badge (`v-if="s.min_input_tokens != null"`, outline, color orange, `>=Nk tokens` label using `formatTokenThreshold`) ← (verify: badge appears/hides correctly, edit dialog pre-populates and clears correctly, save sends correct payload)

## 7. Final Check

- [x] 7.1 Run `just check` (type-check + lint for both frontend and backend) and fix all reported errors ← (verify: zero errors, zero warnings from `just check`)
