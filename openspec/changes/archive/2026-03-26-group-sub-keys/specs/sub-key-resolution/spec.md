## ADDED Requirements

### Requirement: Sub-key resolves to parent group config
The proxy SHALL resolve sub-keys from the `group_keys` table to the parent group's configuration. When a sub-key is used, the proxy SHALL route the request identically to the master key, using the group's configured servers and settings.

#### Scenario: Sub-key cache miss — load from DB
- **WHEN** a proxy request arrives with `x-api-key: sk-vibervn-subkey` and no Redis cache entry exists, and the key does not match any `groups.api_key`, but matches a `group_keys.api_key`
- **THEN** the system SHALL load the parent group's full config (same as master key resolution), set `group_key_id` to the sub-key's UUID, cache it in Redis as `group:config:sk-vibervn-subkey`, and use it for routing

#### Scenario: Sub-key cache hit
- **WHEN** a proxy request arrives with a sub-key that has a Redis cache entry
- **THEN** the system SHALL use the cached config including the `group_key_id` without querying PostgreSQL

#### Scenario: Disabled sub-key
- **WHEN** a proxy request arrives with a sub-key where `group_keys.is_active = false`
- **THEN** the system SHALL return HTTP 403 with `"API key is disabled"`

#### Scenario: Disabled group with active sub-key
- **WHEN** a sub-key is active but its parent group has `is_active = false`
- **THEN** the system SHALL return HTTP 403 with `"API key is disabled"` (group-level check takes precedence)

#### Scenario: Sub-key does not support dynamic keys
- **WHEN** a proxy request arrives with a raw key containing `-rsv-` segments, and the base portion does not match any `groups.api_key`
- **THEN** the system SHALL treat the entire raw key as a plain lookup against both `groups.api_key` and `group_keys.api_key` (no dynamic key extraction)

#### Scenario: Unknown key
- **WHEN** a key does not match any `groups.api_key` or `group_keys.api_key`
- **THEN** the system SHALL return HTTP 401 with `"Invalid API key"`
