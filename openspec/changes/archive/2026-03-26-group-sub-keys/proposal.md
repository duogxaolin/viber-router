## Why

Groups currently have a single API key. For SaaS use cases, each end-user (customer) needs their own key so usage can be tracked, reported, and eventually rate-limited per customer. Multiple keys per group enables this without duplicating group configuration.

## What Changes

- New `group_keys` table allowing multiple sub-keys per group (1:N relationship)
- Proxy resolves sub-keys to the parent group, routing identically to the master key
- Usage tracking records which sub-key was used (`group_key_id` in `token_usage_logs`)
- Admin CRUD API for managing sub-keys within a group
- Frontend "Keys" tab in GroupDetailPage with searchable, paginated table and expandable per-key usage rows
- Cache invalidation extended to cover sub-keys when group config changes
- Schema includes nullable quota columns (`monthly_token_limit`, `monthly_request_limit`) for future rate-limiting — not enforced in this phase

## Capabilities

### New Capabilities
- `group-sub-keys`: CRUD management of sub-keys within a group (create, list, update name/status, regenerate)
- `sub-key-resolution`: Proxy resolution of sub-keys to parent group config, with active/inactive enforcement
- `sub-key-usage-tracking`: Recording group_key_id in token usage logs and querying usage per sub-key
- `sub-key-ui`: Frontend Keys tab with search, pagination, expandable usage rows per key

### Modified Capabilities
- `config-cache`: Cache invalidation must cover all sub-keys when group config changes
- `token-usage-storage`: Add `group_key_id` column to `token_usage_logs`
- `token-usage-stats-api`: Support filtering by `group_key_id`

## Impact

- **Database**: Two new migrations (group_keys table, token_usage_logs column)
- **Backend**: New model, new admin routes, proxy resolution change, cache invalidation update, usage buffer update
- **Frontend**: GroupDetailPage restructured with q-tabs, new Keys tab component, groups store extended
- **API**: New endpoints under `/api/admin/groups/:id/keys`
- **Proxy**: Fallback lookup in group_keys when master key not found — sub-keys do NOT support dynamic keys
