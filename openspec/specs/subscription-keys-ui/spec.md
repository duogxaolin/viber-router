### Requirement: Subscriptions section in expanded sub-key row
The expanded sub-key row in GroupDetailPage Keys tab SHALL display a "Subscriptions" section with a server-side paginated table. The table SHALL show pagination controls (page navigation and rows-per-page) and fetch data from the paginated endpoint.

#### Scenario: Display subscriptions with pagination
- **WHEN** the user expands a sub-key row
- **THEN** the system SHALL fetch the first page of subscriptions from GET `/api/admin/groups/:id/keys/:key_id/subscriptions?page=1&limit=10` and display them in a table with columns: Type, Budget, Used, Status, Duration, Actions, with pagination controls visible

#### Scenario: Navigate pages
- **WHEN** the user clicks a page navigation control in the subscriptions table
- **THEN** the system SHALL fetch the requested page from the server and update the table

#### Scenario: No subscriptions
- **WHEN** a sub-key has no subscriptions (total is 0)
- **THEN** the system SHALL display "No subscriptions — unlimited usage"

### Requirement: Add subscription button
The subscriptions section SHALL include an "Add Subscription" button that opens a dropdown of active plans.

#### Scenario: Add subscription
- **WHEN** the admin clicks "Add Subscription" and selects a plan from the dropdown
- **THEN** the system SHALL POST to `/api/admin/groups/:id/keys/:key_id/subscriptions` with `{ "plan_id": "<uuid>" }` and add the new subscription to the table

#### Scenario: No active plans
- **WHEN** there are no active plans
- **THEN** the dropdown SHALL be empty with a message "No active plans available"

### Requirement: Cancel subscription action
Each active subscription row SHALL have a "Cancel" action button.

#### Scenario: Cancel subscription
- **WHEN** the admin clicks "Cancel" on an active subscription
- **THEN** the system SHALL PATCH `/api/admin/groups/:id/keys/:key_id/subscriptions/:sub_id` with `{ "status": "cancelled" }` and update the row status

#### Scenario: Terminal subscription
- **WHEN** a subscription is in status exhausted, expired, or cancelled
- **THEN** the "Cancel" button SHALL NOT be shown

### Requirement: Reload current page after mutation
After adding or cancelling a subscription, the system SHALL reload the current page of the subscriptions table for that key, preserving the user's pagination position.

#### Scenario: Add subscription reloads current page
- **WHEN** the admin adds a subscription via the "Add Subscription" dropdown
- **THEN** the system SHALL reload the current page of the subscriptions table for that key

#### Scenario: Cancel subscription reloads current page
- **WHEN** the admin cancels a subscription
- **THEN** the system SHALL reload the current page of the subscriptions table for that key

### Requirement: Subscription usage display
Each subscription row SHALL display the current cost usage against the budget.

#### Scenario: Fixed subscription usage
- **WHEN** a fixed subscription has used $340 of $1000
- **THEN** the Used column SHALL display "$340.00 / $1000.00"

#### Scenario: Hourly subscription usage
- **WHEN** an hourly_reset subscription has used $45 of $100 in the current window
- **THEN** the Used column SHALL display "$45.00 / $100.00 (window)"

### Requirement: Subscription status badges
Each subscription status SHALL be displayed with a colored badge: active (green), exhausted (red), expired (orange), cancelled (grey).

#### Scenario: Status display
- **WHEN** a subscription has status "active"
- **THEN** the status SHALL be displayed as a green badge with text "active"
