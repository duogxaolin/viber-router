## MODIFIED Requirements

### Requirement: Token usage log table
The system SHALL persist token usage records in a partitioned `token_usage_logs` PostgreSQL table with columns: `id` (UUID), `created_at` (TIMESTAMPTZ), `group_id` (UUID), `server_id` (UUID), `model` (TEXT, nullable), `input_tokens` (INTEGER), `output_tokens` (INTEGER), `cache_creation_tokens` (INTEGER, nullable), `cache_read_tokens` (INTEGER, nullable), `is_dynamic_key` (BOOLEAN), `key_hash` (TEXT, nullable), `group_key_id` (UUID, nullable), partitioned by range on `created_at`.

#### Scenario: Usage record persisted
- **WHEN** the proxy extracts token usage from a successful `/v1/messages` response
- **THEN** the system SHALL insert a record into `token_usage_logs` with the group_id, server_id, model, input_tokens, output_tokens, cache token counts, is_dynamic_key flag, key_hash, and group_key_id

#### Scenario: Dynamic key usage
- **WHEN** the request used a dynamic key (via `-rsv-` syntax) for the winning server
- **THEN** the record SHALL have `is_dynamic_key: true` and `key_hash` set to the first 16 hex characters of the SHA-256 hash of the dynamic key

#### Scenario: Server default key usage
- **WHEN** the request used the server's default API key (no dynamic key for that server)
- **THEN** the record SHALL have `is_dynamic_key: false` and `key_hash` set to the first 16 hex characters of the SHA-256 hash of the server's default key

#### Scenario: Sub-key usage
- **WHEN** the request was made using a sub-key with id `abc-123`
- **THEN** the record SHALL have `group_key_id = abc-123`

#### Scenario: Master key usage
- **WHEN** the request was made using the group's master key
- **THEN** the record SHALL have `group_key_id = NULL`

### Requirement: Table indexes
The system SHALL create indexes on `token_usage_logs` to support efficient aggregation queries.

#### Scenario: Indexes created
- **WHEN** the migration runs
- **THEN** the system SHALL create indexes on `(group_id, created_at)`, `(key_hash, created_at)`, and `(group_key_id, created_at)`
