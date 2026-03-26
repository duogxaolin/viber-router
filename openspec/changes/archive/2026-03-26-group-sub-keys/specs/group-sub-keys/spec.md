## ADDED Requirements

### Requirement: Group sub-keys table
The system SHALL store sub-keys in a `group_keys` table with columns: `id` (UUID PK), `group_id` (UUID FK to groups), `api_key` (TEXT UNIQUE, format `sk-vibervn-` + 24 random alphanumeric), `name` (TEXT NOT NULL, max 100 characters), `is_active` (BOOLEAN DEFAULT true), `monthly_token_limit` (BIGINT nullable), `monthly_request_limit` (BIGINT nullable), `created_at` (TIMESTAMPTZ), `updated_at` (TIMESTAMPTZ). Indexes SHALL exist on `(api_key)` and `(group_id)`.

#### Scenario: Table structure
- **WHEN** the migration runs
- **THEN** the `group_keys` table SHALL be created with all specified columns, a UNIQUE constraint on `api_key`, a FOREIGN KEY on `group_id` referencing `groups(id)` with CASCADE delete, and indexes on `api_key` and `group_id`

### Requirement: Create sub-key
The system SHALL allow creating a sub-key for a group via POST `/api/admin/groups/:group_id/keys` with body `{ "name": "<string>" }`. The API key SHALL be auto-generated using the same format as group master keys (`sk-vibervn-` + 24 random alphanumeric). The sub-key SHALL default to `is_active: true`.

#### Scenario: Successful sub-key creation
- **WHEN** an authenticated admin sends POST `/api/admin/groups/:group_id/keys` with `{"name": "Customer A"}`
- **THEN** the system SHALL create a sub-key with an auto-generated API key and return HTTP 201 with the created sub-key object (id, group_id, api_key, name, is_active, created_at, updated_at)

#### Scenario: Group not found
- **WHEN** the group_id does not exist
- **THEN** the system SHALL return HTTP 404 with "Group not found"

#### Scenario: Name exceeds max length
- **WHEN** the name exceeds 100 characters
- **THEN** the system SHALL return HTTP 400 with "Name must be 100 characters or less"

### Requirement: List sub-keys
The system SHALL allow listing sub-keys for a group via GET `/api/admin/groups/:group_id/keys` with pagination (`page`, `limit`) and optional search by name (`search` query parameter). Results SHALL be ordered by `created_at` descending.

#### Scenario: List with pagination
- **WHEN** an authenticated admin sends GET `/api/admin/groups/:group_id/keys?page=1&limit=50`
- **THEN** the system SHALL return a paginated response with `data` (array of sub-keys), `total`, `page`, `total_pages`

#### Scenario: Search by name
- **WHEN** the request includes `search=customer`
- **THEN** the system SHALL filter sub-keys where `name` ILIKE `%customer%`

#### Scenario: Default pagination
- **WHEN** no pagination parameters are provided
- **THEN** the system SHALL default to page 1, limit 50

### Requirement: Update sub-key
The system SHALL allow updating a sub-key's name and active status via PATCH `/api/admin/groups/:group_id/keys/:key_id` with body `{ "name": "<string>", "is_active": <bool> }`. Both fields are optional.

#### Scenario: Deactivate sub-key
- **WHEN** an authenticated admin sends PATCH with `{"is_active": false}`
- **THEN** the system SHALL set `is_active` to false, update `updated_at`, invalidate the Redis cache entry for this sub-key's api_key, and return the updated sub-key

#### Scenario: Rename sub-key
- **WHEN** an authenticated admin sends PATCH with `{"name": "New Name"}`
- **THEN** the system SHALL update the name and `updated_at`, and return the updated sub-key

#### Scenario: Sub-key not found
- **WHEN** the key_id does not exist or does not belong to the group
- **THEN** the system SHALL return HTTP 404 with "Key not found"

### Requirement: Regenerate sub-key
The system SHALL allow regenerating a sub-key's API key via POST `/api/admin/groups/:group_id/keys/:key_id/regenerate`. The old key SHALL be invalidated in Redis cache and a new key generated.

#### Scenario: Successful regeneration
- **WHEN** an authenticated admin sends POST to the regenerate endpoint
- **THEN** the system SHALL generate a new API key, update the sub-key record, invalidate the old key's Redis cache entry, and return the updated sub-key with the new API key
