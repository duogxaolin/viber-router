## 1. Backend — Paginate list_subscriptions

- [x] 1.1 Add `ListParams` struct (page, limit) and `Query<ListParams>` extraction to `list_subscriptions` in `viber-router-api/src/routes/admin/key_subscriptions.rs`
- [x] 1.2 Add COUNT query for total subscriptions matching `group_key_id`
- [x] 1.3 Add LIMIT/OFFSET to the SELECT query, clamp limit to 1..100, default page=1 limit=10
- [x] 1.4 Return `PaginatedResponse<KeySubscriptionWithUsage>` instead of `Vec<KeySubscriptionWithUsage>` ← (verify: endpoint returns `{ data, total, page, total_pages }`, pagination params work correctly, limit clamped to 100)

## 2. Frontend — Server-side pagination for subscriptions table

- [x] 2.1 Change `keySubscriptions` from `Record<string, KeySubscription[]>` to `Record<string, { data: KeySubscription[], total: number }>` in `GroupDetailPage.vue`
- [x] 2.2 Add per-key pagination state `subPagination` as `Record<string, { page: number, rowsPerPage: number, rowsNumber: number }>`
- [x] 2.3 Update `loadKeySubscriptions(keyId, page?, limit?)` to pass `?page=&limit=` and store paginated response
- [x] 2.4 Update subscriptions q-table: remove `hide-pagination`, bind `:pagination` to per-key state, add `@request` handler for server-side pagination
- [x] 2.5 Update template references from `keySubscriptions[id]` to `keySubscriptions[id]?.data` and empty check against total
- [x] 2.6 Update `onAssignSubscription` and `onCancelSubscription` to reload current page after mutation ← (verify: pagination controls visible, page navigation works, current page preserved after add/cancel, empty state still shows correctly)

## 3. Validation

- [x] 3.1 Run `just check` — fix all type and lint errors ← (verify: zero errors from `just check`)
