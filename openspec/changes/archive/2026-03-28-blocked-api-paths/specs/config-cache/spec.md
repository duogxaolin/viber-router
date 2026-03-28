## MODIFIED Requirements

### Requirement: Write-through cache invalidation
The system SHALL invalidate relevant Redis cache entries immediately when admin operations modify group configuration or global settings.

#### Scenario: Group updated
- **WHEN** a group's name, failover_status_codes, is_active, count_tokens_server_id, or count_tokens_model_mappings is updated
- **THEN** the Redis cache entry for that group's API key SHALL be deleted

#### Scenario: Group API key regenerated
- **WHEN** a group's API key is regenerated from `sk-vibervn-old` to `sk-vibervn-new`
- **THEN** the Redis cache entry for `sk-vibervn-old` SHALL be deleted

#### Scenario: Group-server assignment changed
- **WHEN** a server is added to, removed from, or reordered within a group
- **THEN** the Redis cache entry for that group's API key SHALL be deleted

#### Scenario: Server updated
- **WHEN** a server's base_url or api_key is updated
- **THEN** the Redis cache entries for ALL groups that reference this server (via `group_servers` OR via `count_tokens_server_id`) SHALL be deleted

#### Scenario: Group deleted
- **WHEN** a group is deleted
- **THEN** the Redis cache entry for that group's API key SHALL be deleted

#### Scenario: Settings blocked paths updated
- **WHEN** an admin updates settings via PUT `/api/admin/settings` and the request includes `blocked_paths`
- **THEN** the Redis key `settings:blocked_paths` SHALL be deleted so the next proxy request reloads from the database
