## MODIFIED Requirements

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

### Requirement: Reload current page after mutation
After adding or cancelling a subscription, the system SHALL reload the current page of the subscriptions table for that key, preserving the user's pagination position.

#### Scenario: Add subscription reloads current page
- **WHEN** the admin adds a subscription via the "Add Subscription" dropdown
- **THEN** the system SHALL reload the current page of the subscriptions table for that key

#### Scenario: Cancel subscription reloads current page
- **WHEN** the admin cancels a subscription
- **THEN** the system SHALL reload the current page of the subscriptions table for that key
