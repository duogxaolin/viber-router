## ADDED Requirements

### Requirement: Group-server assignment has active hours fields
The `group_servers` table SHALL include three nullable TEXT columns: `active_hours_start`, `active_hours_end`, and `active_hours_timezone`. All three default to NULL. A NULL set means the server is active 24/7. The fields SHALL be included in `GroupServerDetail` and `AdminGroupServerDetail` responses. All three fields SHALL carry `#[serde(default)]` in Rust models to ensure backward compatibility when deserializing cached Redis data that predates this change.

#### Scenario: New assignment defaults to no active hours restriction
- **WHEN** an admin assigns a server to a group without specifying active hours fields
- **THEN** the assignment SHALL have `active_hours_start=NULL`, `active_hours_end=NULL`, and `active_hours_timezone=NULL`

#### Scenario: AdminGroupServerDetail includes active hours fields
- **WHEN** the admin fetches a group detail via GET `/api/admin/groups/{group_id}`
- **THEN** each server in the `servers` array SHALL include `active_hours_start`, `active_hours_end`, and `active_hours_timezone` (all nullable strings)

#### Scenario: Existing cached GroupConfig deserializes with new fields absent
- **WHEN** a `GroupConfig` object cached in Redis before this deployment is deserialized
- **THEN** the three active hours fields SHALL default to `None` (i.e., 24/7 behavior) without a deserialization error

### Requirement: Update active hours configuration via assignment endpoint
The system SHALL allow setting active hours fields via PUT `/api/admin/groups/{group_id}/servers/{server_id}`. All three fields MUST be either all provided (non-null) or all null (all-or-nothing). The `active_hours_start` and `active_hours_end` values MUST match the pattern `HH:MM` where HH is 00-23 and MM is 00-59. The `active_hours_timezone` value MUST be a valid IANA timezone string recognized by the `chrono-tz` crate. Omitting all three fields leaves the existing values unchanged. On successful update, the system SHALL call `invalidate_group_all_keys()` to clear the Redis cache.

#### Scenario: Set active hours — all three fields provided and valid
- **WHEN** admin sends PUT with `{"active_hours_start": "08:00", "active_hours_end": "23:00", "active_hours_timezone": "Asia/Ho_Chi_Minh"}`
- **THEN** the system SHALL update all three fields, invalidate the group's Redis cache, and return the updated assignment

#### Scenario: Clear active hours — all three null
- **WHEN** admin sends PUT with `{"active_hours_start": null, "active_hours_end": null, "active_hours_timezone": null}`
- **THEN** the system SHALL set all three fields to NULL, invalidate the group's Redis cache, and return the updated assignment

#### Scenario: Omit active hours fields — no change
- **WHEN** admin sends PUT without any active hours fields
- **THEN** the system SHALL leave the existing active hours values unchanged

#### Scenario: Partial active hours config — validation error
- **WHEN** admin sends PUT with `{"active_hours_start": "08:00", "active_hours_end": "23:00"}` (missing timezone)
- **THEN** the system SHALL return HTTP 400 with error message explaining all-or-nothing requirement

#### Scenario: Invalid time format — validation error
- **WHEN** admin sends PUT with `{"active_hours_start": "8:00", "active_hours_end": "23:00", "active_hours_timezone": "UTC"}`
- **THEN** the system SHALL return HTTP 400 with error message indicating the time format must be HH:MM

#### Scenario: Invalid timezone string — validation error
- **WHEN** admin sends PUT with `{"active_hours_start": "08:00", "active_hours_end": "23:00", "active_hours_timezone": "Not/ATimezone"}`
- **THEN** the system SHALL return HTTP 400 with error message indicating the timezone is not a recognized IANA timezone
