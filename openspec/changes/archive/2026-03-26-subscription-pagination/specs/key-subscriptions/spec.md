## MODIFIED Requirements

### Requirement: List subscriptions for a sub-key
The system SHALL return paginated subscriptions for a sub-key via GET `/api/admin/groups/:group_id/keys/:key_id/subscriptions` with optional query parameters `page` (default 1) and `limit` (default 10, max 100). The response SHALL be a JSON object with fields `data` (array of subscriptions with `cost_used`), `total` (total count), `page` (current page), and `total_pages` (computed ceiling of total/limit).

#### Scenario: List subscriptions with default pagination
- **WHEN** an admin sends GET for a sub-key's subscriptions without query parameters
- **THEN** the system SHALL return the first 10 subscriptions ordered by `created_at` descending, wrapped in `{ data, total, page: 1, total_pages }`

#### Scenario: List subscriptions with explicit page and limit
- **WHEN** an admin sends GET with `?page=2&limit=5`
- **THEN** the system SHALL return subscriptions 6-10 (offset 5) ordered by `created_at` descending, with correct `total` and `total_pages` values

#### Scenario: Page beyond available data
- **WHEN** an admin sends GET with a page number exceeding available pages
- **THEN** the system SHALL return an empty `data` array with correct `total` and `total_pages`

#### Scenario: Limit clamping
- **WHEN** an admin sends GET with `limit=500`
- **THEN** the system SHALL clamp the limit to 100
