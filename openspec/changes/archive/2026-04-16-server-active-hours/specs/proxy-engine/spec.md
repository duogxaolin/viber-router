## ADDED Requirements

### Requirement: Active hours skip condition in proxy failover
Before attempting each server in the failover waterfall, after existing skip conditions (circuit breaker, rate limit, token thresholds, per-server model filter), the proxy SHALL evaluate the server's active hours configuration. If all three fields (`active_hours_start`, `active_hours_end`, `active_hours_timezone`) are present (non-null), the proxy SHALL parse the timezone using the `chrono-tz` crate, obtain the current local time in that timezone, and determine whether the current time falls within the configured window. If the current time is outside the window, the proxy SHALL skip the server and continue to the next one. If any of the three fields is absent, or if the timezone string cannot be parsed, the proxy SHALL skip the active hours check and treat the server as always active (fail-open), emitting a `warn!` log if the timezone is unparseable.

#### Scenario: Current time within active hours window — server attempted
- **WHEN** a server has `active_hours_start="08:00"`, `active_hours_end="23:00"`, `active_hours_timezone="Asia/Ho_Chi_Minh"`, and the current time in that timezone is 14:30
- **THEN** the proxy SHALL proceed to attempt the server (active hours check passes)

#### Scenario: Current time outside active hours window — server skipped
- **WHEN** a server has `active_hours_start="08:00"`, `active_hours_end="23:00"`, `active_hours_timezone="Asia/Ho_Chi_Minh"`, and the current time in that timezone is 01:00
- **THEN** the proxy SHALL skip this server without sending a request and continue to the next server in the waterfall

#### Scenario: Overnight window — current time after start
- **WHEN** a server has `active_hours_start="22:00"`, `active_hours_end="06:00"`, and the current time in the configured timezone is 23:30
- **THEN** the proxy SHALL treat the server as active (start > end indicates overnight window; 23:30 >= 22:00)

#### Scenario: Overnight window — current time before end
- **WHEN** a server has `active_hours_start="22:00"`, `active_hours_end="06:00"`, and the current time in the configured timezone is 03:00
- **THEN** the proxy SHALL treat the server as active (03:00 <= 06:00)

#### Scenario: Overnight window — current time in inactive gap
- **WHEN** a server has `active_hours_start="22:00"`, `active_hours_end="06:00"`, and the current time in the configured timezone is 12:00
- **THEN** the proxy SHALL skip this server (12:00 is outside 22:00-06:00 overnight window)

#### Scenario: Incomplete active hours config — fail open
- **WHEN** a server has `active_hours_start="08:00"` but `active_hours_end=NULL` and `active_hours_timezone=NULL`
- **THEN** the proxy SHALL skip the active hours check and treat the server as always active

#### Scenario: Unparseable timezone — fail open with warning
- **WHEN** a server has all three active hours fields set but `active_hours_timezone` contains a string not recognized by `chrono-tz`
- **THEN** the proxy SHALL skip the active hours check, treat the server as always active, and emit a `warn!` log message

#### Scenario: Active hours config is NULL — no check performed
- **WHEN** a server has all three active hours fields set to NULL
- **THEN** the proxy SHALL not perform any active hours check and SHALL attempt the server normally

### Requirement: Active hours window interpretation
The active hours window SHALL be interpreted as follows:
- If `active_hours_start <= active_hours_end` (same-day window): the server is active when `start <= current_time <= end`
- If `active_hours_start > active_hours_end` (overnight window): the server is active when `current_time >= start OR current_time <= end`

Time comparison SHALL be performed at minute granularity (HH:MM), ignoring seconds.

#### Scenario: Boundary — current time equals start
- **WHEN** current time equals `active_hours_start` exactly (e.g., both are 08:00)
- **THEN** the proxy SHALL treat the server as active (inclusive boundary)

#### Scenario: Boundary — current time equals end
- **WHEN** current time equals `active_hours_end` exactly (e.g., both are 23:00)
- **THEN** the proxy SHALL treat the server as active (inclusive boundary)
