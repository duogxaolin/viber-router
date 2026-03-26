## ADDED Requirements

### Requirement: Keys tab in GroupDetailPage
The GroupDetailPage SHALL be restructured with `q-tabs` navigation containing tabs: Properties, Servers, Keys, TTFT, Token Usage. The Keys tab SHALL display a searchable, paginated table of sub-keys with expandable rows showing per-key usage.

#### Scenario: Tab navigation
- **WHEN** a user views the GroupDetailPage
- **THEN** the page SHALL display q-tabs at the top with Properties, Servers, Keys, TTFT, Token Usage tabs, defaulting to the Properties tab

#### Scenario: Keys tab — list with search
- **WHEN** a user navigates to the Keys tab
- **THEN** the system SHALL display a search input, a "Create Key" button, and a paginated table with columns: Name, Key (masked with copy button), Status (active/inactive toggle), Actions (regenerate)

#### Scenario: Keys tab — pagination
- **WHEN** the group has more than 50 sub-keys
- **THEN** the table SHALL paginate with 50 rows per page, showing total count and page navigation

#### Scenario: Keys tab — search
- **WHEN** the user types in the search input
- **THEN** the table SHALL filter sub-keys by name (server-side search via API)

#### Scenario: Keys tab — expandable usage row
- **WHEN** the user clicks to expand a sub-key row
- **THEN** the system SHALL fetch and display token usage for that sub-key in a nested table with columns: Server, Model, Input Tokens, Output Tokens, Cache Creation, Cache Read, Requests

#### Scenario: Keys tab — create sub-key
- **WHEN** the user clicks "Create Key" and enters a name
- **THEN** the system SHALL call POST `/api/admin/groups/:id/keys` and add the new key to the table

#### Scenario: Keys tab — toggle active status
- **WHEN** the user toggles a sub-key's active status
- **THEN** the system SHALL call PATCH `/api/admin/groups/:id/keys/:key_id` with the new `is_active` value

#### Scenario: Keys tab — regenerate key
- **WHEN** the user clicks regenerate on a sub-key and confirms the dialog
- **THEN** the system SHALL call POST `/api/admin/groups/:id/keys/:key_id/regenerate` and update the displayed key

#### Scenario: Keys tab — copy key
- **WHEN** the user clicks the copy button next to a sub-key
- **THEN** the full API key SHALL be copied to clipboard with a success notification

#### Scenario: Keys tab — empty state
- **WHEN** the group has no sub-keys
- **THEN** the Keys tab SHALL display "No sub-keys created" with a prompt to create one
