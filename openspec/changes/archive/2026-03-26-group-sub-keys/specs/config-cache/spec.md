## MODIFIED Requirements

### Requirement: Write-through cache invalidation
The system SHALL invalidate relevant Redis cache entries immediately when admin operations modify group configuration. This includes cache entries for both the group's master key AND all sub-keys belonging to the group.

#### Scenario: Group updated
- **WHEN** a group's name, failover_status_codes, is_active, count_tokens_server_id, or count_tokens_model_mappings is updated
- **THEN** the Redis cache entry for that group's master API key AND all sub-key API keys SHALL be deleted

#### Scenario: Group API key regenerated
- **WHEN** a group's API key is regenerated from `sk-vibervn-old` to `sk-vibervn-new`
- **THEN** the Redis cache entry for `sk-vibervn-old` SHALL be deleted

#### Scenario: Group-server assignment changed
- **WHEN** a server is added to, removed from, or reordered within a group
- **THEN** the Redis cache entries for that group's master API key AND all sub-key API keys SHALL be deleted

#### Scenario: Server updated
- **WHEN** a server's base_url or api_key is updated
- **THEN** the Redis cache entries for ALL groups that reference this server (via `group_servers` OR via `count_tokens_server_id`) SHALL be deleted, including all sub-key cache entries for those groups

#### Scenario: Group deleted
- **WHEN** a group is deleted
- **THEN** the Redis cache entries for that group's master API key AND all sub-key API keys SHALL be deleted

#### Scenario: Sub-key deactivated or regenerated
- **WHEN** a sub-key's `is_active` is changed or its API key is regenerated
- **THEN** the Redis cache entry for that specific sub-key's API key SHALL be deleted
