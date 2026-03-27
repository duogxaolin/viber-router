## Why

The subscriptions list in the expanded sub-key row currently fetches all subscriptions at once with no pagination. As keys accumulate subscriptions over time (active, cancelled, expired, exhausted), the list grows unbounded. Server-side pagination is needed to keep the UI responsive and reduce payload size.

## What Changes

- Backend `list_subscriptions` handler gains `page` and `limit` query parameters and returns `PaginatedResponse<KeySubscriptionWithUsage>` instead of `Vec<KeySubscriptionWithUsage>`
- Frontend subscriptions q-table switches from client-side hidden pagination to server-side pagination with `@request` handler
- Frontend state tracks pagination metadata per key
- After add/cancel subscription, the current page is reloaded (not reset to page 1)

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `subscription-keys-ui`: Subscriptions table gains server-side pagination (page controls, rows-per-page selector)
- `key-subscriptions`: List endpoint returns paginated response with `page`, `limit` query params

## Impact

- `viber-router-api/src/routes/admin/key_subscriptions.rs` — `list_subscriptions` handler changes return type and adds query params
- `src/pages/GroupDetailPage.vue` — subscriptions q-table, `loadKeySubscriptions`, `keySubscriptions` state, add/cancel reload logic
