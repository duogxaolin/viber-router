## ADDED Requirements

### Requirement: Proxy skips servers below min_input_tokens threshold
During the failover waterfall, for each server that has `min_input_tokens` set (non-null), the proxy SHALL compare the estimated token count to the threshold. If the estimated token count is less than the server's `min_input_tokens`, the proxy SHALL skip that server and continue to the next. If `min_input_tokens` is NULL or no estimate is available, the proxy SHALL not skip the server based on this check. This gate is evaluated independently of and in addition to the `max_input_tokens` gate.

#### Scenario: Estimated tokens below minimum — server skipped
- **WHEN** a server has `min_input_tokens=10000` and the estimated token count is 5000
- **THEN** the proxy SHALL skip this server without sending a request and continue to the next server in the waterfall

#### Scenario: Estimated tokens at minimum — proceed
- **WHEN** a server has `min_input_tokens=10000` and the estimated token count is exactly 10000
- **THEN** the proxy SHALL NOT skip this server (the threshold is only triggered when strictly less than)

#### Scenario: Estimated tokens above minimum — proceed
- **WHEN** a server has `min_input_tokens=10000` and the estimated token count is 15000
- **THEN** the proxy SHALL NOT skip this server based on the minimum token threshold

#### Scenario: min_input_tokens NULL — no skip
- **WHEN** a server has `min_input_tokens=NULL`
- **THEN** the proxy SHALL NOT skip this server based on the minimum token count regardless of request size

#### Scenario: Estimation absent — no token-based skip
- **WHEN** the estimated token count is absent (e.g., non-JSON body) and a server has `min_input_tokens=10000`
- **THEN** the proxy SHALL NOT skip this server based on the minimum token threshold

#### Scenario: Both min and max set — both gates apply independently
- **WHEN** a server has `min_input_tokens=5000` and `max_input_tokens=50000` and the estimated token count is 3000
- **THEN** the proxy SHALL skip this server because 3000 < 5000 (min gate triggers)

#### Scenario: Both min and max set — request in range — proceed
- **WHEN** a server has `min_input_tokens=5000` and `max_input_tokens=50000` and the estimated token count is 20000
- **THEN** the proxy SHALL NOT skip this server (20000 is within [5000, 50000])

### Requirement: min_input_tokens is persisted per group server assignment
The `group_servers` table SHALL have a nullable integer column `min_input_tokens`. The admin API SHALL accept `min_input_tokens` in both the assign-server and update-assignment payloads. The value SHALL be stored and returned in all group server detail responses.

#### Scenario: Assign server with min_input_tokens set
- **WHEN** an admin assigns a server to a group with `min_input_tokens=8000`
- **THEN** the assignment SHALL be stored with `min_input_tokens=8000` and returned in subsequent detail queries

#### Scenario: Assign server without min_input_tokens
- **WHEN** an admin assigns a server to a group without providing `min_input_tokens`
- **THEN** the assignment SHALL be stored with `min_input_tokens=NULL`

#### Scenario: Update assignment to set min_input_tokens
- **WHEN** an admin updates an existing assignment and provides `min_input_tokens=12000`
- **THEN** the stored value SHALL be updated to 12000

#### Scenario: Update assignment to clear min_input_tokens
- **WHEN** an admin updates an existing assignment and provides `min_input_tokens=null`
- **THEN** the stored value SHALL be set to NULL

#### Scenario: Update assignment without min_input_tokens field
- **WHEN** an admin updates an existing assignment and omits the `min_input_tokens` field entirely
- **THEN** the stored `min_input_tokens` value SHALL remain unchanged

### Requirement: Admin UI exposes min_input_tokens in group detail
The group detail page SHALL display a `min_input_tokens` input field below the existing `max_input_tokens` field in the server edit dialog. When a server has `min_input_tokens` set, the server list SHALL show a badge indicating the minimum threshold. Clearing the field SHALL send NULL to the API.

#### Scenario: Badge shown when min_input_tokens is set
- **WHEN** a server assignment has `min_input_tokens=8000`
- **THEN** the server list SHALL display a badge showing the minimum threshold (e.g., ">=8K tokens")

#### Scenario: No badge when min_input_tokens is null
- **WHEN** a server assignment has `min_input_tokens=NULL`
- **THEN** the server list SHALL NOT display a minimum token badge for that server

#### Scenario: Edit dialog pre-populates min_input_tokens
- **WHEN** an admin opens the edit dialog for a server with `min_input_tokens=8000`
- **THEN** the min input tokens field SHALL be pre-filled with 8000

#### Scenario: Clearing the field sends null
- **WHEN** an admin clears the min input tokens field and saves
- **THEN** the update payload SHALL include `min_input_tokens: null`
