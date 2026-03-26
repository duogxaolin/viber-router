## ADDED Requirements

### Requirement: Record group_key_id in token usage logs
The system SHALL record the `group_key_id` (UUID, nullable) in `token_usage_logs` when a sub-key is used for a proxy request. When the master key is used, `group_key_id` SHALL be NULL.

#### Scenario: Sub-key usage recorded
- **WHEN** a proxy request using sub-key with id `abc-123` completes with token usage data
- **THEN** the `token_usage_logs` entry SHALL have `group_key_id = abc-123`

#### Scenario: Master key usage recorded
- **WHEN** a proxy request using the group's master key completes with token usage data
- **THEN** the `token_usage_logs` entry SHALL have `group_key_id = NULL`

### Requirement: Query usage per sub-key
The token usage stats API SHALL support filtering by `group_key_id` to return usage for a specific sub-key.

#### Scenario: Filter by group_key_id
- **WHEN** an authenticated admin sends GET `/api/admin/token-usage?group_id=<uuid>&group_key_id=<uuid>&period=24h`
- **THEN** the system SHALL return aggregated statistics only for records matching that `group_key_id`

#### Scenario: Master key usage filter
- **WHEN** an authenticated admin sends GET `/api/admin/token-usage?group_id=<uuid>&group_key_id=null&period=24h`
- **THEN** the system SHALL return aggregated statistics only for records where `group_key_id IS NULL` (master key usage)
