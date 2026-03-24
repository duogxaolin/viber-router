## ADDED Requirements

### Requirement: Settings storage
The system SHALL store Telegram alert configuration in a `settings` table with a single row (id = 1). The table SHALL contain: `telegram_bot_token` (TEXT nullable), `telegram_chat_ids` (TEXT[] NOT NULL DEFAULT '{}'), `alert_status_codes` (INT[] NOT NULL DEFAULT '{500,502,503}'), `alert_cooldown_mins` (INT NOT NULL DEFAULT 5).

#### Scenario: Settings table created by migration
- **WHEN** migration `009_telegram_settings.sql` runs
- **THEN** the `settings` table SHALL exist with the schema above and no rows

#### Scenario: Default settings row created on first GET
- **WHEN** GET `/api/admin/settings` is called and no row exists
- **THEN** the system SHALL return default values: `telegram_bot_token: null`, `telegram_chat_ids: []`, `alert_status_codes: [500, 502, 503]`, `alert_cooldown_mins: 5`

### Requirement: Get settings API
The system SHALL provide GET `/api/admin/settings` (admin-authenticated) that returns the current settings row. If no row exists, it SHALL return defaults without inserting a row.

#### Scenario: Settings exist
- **WHEN** an authenticated admin calls GET `/api/admin/settings` and a settings row exists
- **THEN** the system SHALL return HTTP 200 with the settings JSON

#### Scenario: Settings do not exist yet
- **WHEN** an authenticated admin calls GET `/api/admin/settings` and no row exists
- **THEN** the system SHALL return HTTP 200 with default values (`telegram_bot_token: null`, `telegram_chat_ids: []`, `alert_status_codes: [500, 502, 503]`, `alert_cooldown_mins: 5`)

### Requirement: Update settings API
The system SHALL provide PUT `/api/admin/settings` (admin-authenticated) that upserts the settings row (INSERT ... ON CONFLICT (id) DO UPDATE). All fields are optional in the request body; omitted fields retain their current values.

#### Scenario: Update bot token and chat IDs
- **WHEN** an authenticated admin calls PUT `/api/admin/settings` with `{"telegram_bot_token": "123:abc", "telegram_chat_ids": ["111", "222"]}`
- **THEN** the system SHALL upsert the row and return HTTP 200 with the updated settings

#### Scenario: Update only cooldown
- **WHEN** an authenticated admin calls PUT `/api/admin/settings` with `{"alert_cooldown_mins": 10}`
- **THEN** the system SHALL update only `alert_cooldown_mins`, leaving other fields unchanged, and return HTTP 200

#### Scenario: Clear bot token
- **WHEN** an authenticated admin calls PUT `/api/admin/settings` with `{"telegram_bot_token": null}`
- **THEN** the system SHALL set `telegram_bot_token` to NULL and return HTTP 200

### Requirement: Test alert API
The system SHALL provide POST `/api/admin/settings/test` (admin-authenticated) that sends a test message to all configured `telegram_chat_ids` using the configured `telegram_bot_token`.

#### Scenario: Successful test
- **WHEN** an authenticated admin calls POST `/api/admin/settings/test` and the bot token and chat IDs are valid
- **THEN** the system SHALL send a test message to each chat ID and return HTTP 200 with `{"success": true}`

#### Scenario: No chat IDs configured
- **WHEN** an authenticated admin calls POST `/api/admin/settings/test` and `telegram_chat_ids` is empty
- **THEN** the system SHALL return HTTP 400 with `{"error": "No chat IDs configured"}`

#### Scenario: Bot token missing
- **WHEN** an authenticated admin calls POST `/api/admin/settings/test` and `telegram_bot_token` is null
- **THEN** the system SHALL return HTTP 400 with `{"error": "Bot token not configured"}`

#### Scenario: Telegram API rejects token
- **WHEN** an authenticated admin calls POST `/api/admin/settings/test` and Telegram returns a non-2xx response
- **THEN** the system SHALL return HTTP 502 with `{"error": "<telegram error message>"}`

### Requirement: Admin UI Settings page
The admin UI SHALL provide a `/settings` route with a form to configure Telegram alerts. The sidebar SHALL include a "Settings" navigation item.

#### Scenario: Navigate to settings
- **WHEN** the user clicks "Settings" in the sidebar
- **THEN** the Settings page is displayed with the current configuration loaded from GET `/api/admin/settings`

#### Scenario: Save settings
- **WHEN** the user fills in the form and clicks "Save Settings"
- **THEN** the system SHALL call PUT `/api/admin/settings` and show a success notification on HTTP 200

#### Scenario: Save settings fails
- **WHEN** PUT `/api/admin/settings` returns an error
- **THEN** the system SHALL show an inline error message

#### Scenario: Test alert button
- **WHEN** the user clicks "Test Alert"
- **THEN** the system SHALL call POST `/api/admin/settings/test` and show a success or error notification based on the response
