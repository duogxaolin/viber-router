## Context

Viber Router's proxy engine walks a failover waterfall of servers for each request. The existing `max_input_tokens` column on `group_servers` lets admins exclude servers when a request is too large. There is currently no lower-bound equivalent, so admins cannot reserve high-capacity servers for large requests only.

The token estimation algorithm (strip images, divide byte length by 4) already runs before the waterfall and its result is available to all gate checks. The `max_input_tokens` gate is a single `if` block in `proxy.rs`; `min_input_tokens` follows the identical pattern.

## Goals / Non-Goals

**Goals:**
- Add `min_input_tokens` as a nullable integer column on `group_servers`.
- Extend the proxy routing gate to skip a server when `estimated_tokens < min_input_tokens`.
- Expose the field in the admin API (assign and update endpoints) and in all detail response structs.
- Add UI input and badge in the group detail page, matching the existing `max_input_tokens` UX exactly.

**Non-Goals:**
- Changing the token estimation algorithm.
- Adding any new API versioning or breaking changes to existing payloads.
- Enforcing that `min_input_tokens < max_input_tokens` at the database or API layer (admin responsibility).

## Decisions

**Decision: Mirror max_input_tokens patterns exactly**
Every layer (migration, model structs, SQL binds, proxy gate, frontend type, UI component) follows the same pattern already established for `max_input_tokens`. This minimises review surface and cognitive overhead. No new abstractions are introduced.

Alternatives considered: a single "token range" struct wrapping both fields — rejected because it would require a larger refactor with no benefit at this scale.

**Decision: Double-Option pattern for UpdateAssignment**
The existing `UpdateAssignment` struct uses `Option<Option<i32>>` for `max_input_tokens` to distinguish "field omitted" (outer None → no change) from "field set to null" (outer Some(None) → clear). `min_input_tokens` uses the same pattern for consistency.

**Decision: Fail-open semantics**
When `min_input_tokens` is NULL or when token estimation is absent, the gate does not skip the server. This matches `max_input_tokens` and ensures that misconfiguration or estimation failure never causes a complete routing blackout.

**Decision: No compound range validation**
The API will not validate that `min_input_tokens < max_input_tokens`. If both are set and min >= max, no server in the chain will ever match that range — this is a misconfiguration the admin must avoid. Adding validation would complicate the update path without meaningful safety gain for an internal admin tool.

## Risks / Trade-offs

- [Risk: Misconfigured range locks out all servers] → Mitigation: document the expected invariant in the UI tooltip; the fail-open NULL default means a fresh assignment is always safe.
- [Risk: SQL parameter index drift in update query] → Mitigation: the tasks spec calls out the exact parameter indices to verify; the `just check` step will catch type mismatches at compile time via sqlx.

## Migration Plan

1. Deploy the SQL migration (`036_add_min_input_tokens_to_group_servers.sql`) — adds a nullable column with no default change to existing rows.
2. Deploy the updated backend binary — new field is optional in all payloads; old clients sending no `min_input_tokens` field continue to work.
3. Deploy the updated frontend — new UI field appears; existing assignments show no badge (NULL).

Rollback: drop the column (no data loss risk since it is nullable and new). Revert backend and frontend binaries.
