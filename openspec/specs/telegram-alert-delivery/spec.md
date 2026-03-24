## Requirements

### Requirement: Alert triggered on configured error status codes
The system SHALL send a Telegram alert when a proxy request to `POST /v1/messages` completes with a status code that is in the `alert_status_codes` setting. The alert SHALL be sent after the proxy response is returned (fire-and-forget via `tokio::spawn`).

#### Scenario: Upstream returns 500 and 500 is in alert_status_codes
- **WHEN** a `POST /v1/messages` request results in a final status of 500 and `alert_status_codes` contains 500
- **THEN** the system SHALL spawn an async task to send a Telegram alert and SHALL NOT delay the proxy response

#### Scenario: Upstream returns 429 and 429 is not in alert_status_codes
- **WHEN** a `POST /v1/messages` request results in a final status of 429 and `alert_status_codes` does not contain 429
- **THEN** the system SHALL NOT send any Telegram alert

#### Scenario: Request path is not /v1/messages
- **WHEN** a proxy request to a path other than `/v1/messages` results in an error status
- **THEN** the system SHALL NOT send any Telegram alert

#### Scenario: Bot token not configured
- **WHEN** an error occurs but `telegram_bot_token` is NULL in settings
- **THEN** the system SHALL silently skip alert delivery without logging a warning

### Requirement: Cooldown suppresses duplicate alerts
The system SHALL suppress duplicate alerts for the same `server_id + status_code` combination within the configured `alert_cooldown_mins` window. The cooldown SHALL be tracked in Redis using key `tg:cooldown:{server_id}:{status_code}` with TTL = `alert_cooldown_mins * 60` seconds, set atomically with `SET NX EX`.

#### Scenario: First alert for server+code — sent and cooldown set
- **WHEN** an alert is triggered for server S1 with status 500 and no cooldown key exists
- **THEN** the system SHALL send the alert to all chat IDs AND set Redis key `tg:cooldown:{S1_id}:500` with TTL = `alert_cooldown_mins * 60`

#### Scenario: Duplicate alert within cooldown window — suppressed
- **WHEN** an alert is triggered for server S1 with status 500 and the cooldown key already exists
- **THEN** the system SHALL NOT send any Telegram message

#### Scenario: Alert after cooldown expires — sent again
- **WHEN** an alert is triggered for server S1 with status 500 and the cooldown key has expired
- **THEN** the system SHALL send the alert and reset the cooldown key

#### Scenario: Redis unavailable during cooldown check — fail open
- **WHEN** Redis is unavailable when checking the cooldown key
- **THEN** the system SHALL proceed to send the alert (fail open) and log a warning via `tracing::warn`

### Requirement: Alert message content
Each Telegram alert message SHALL include: server name, group name, HTTP status code, latency in milliseconds, and UTC timestamp. The message SHALL be sent using Telegram `sendMessage` API with `parse_mode=MarkdownV2`.

#### Scenario: Alert message format
- **WHEN** an alert is sent for server "claude-prod" in group "my-group" with status 500 and latency 1234ms
- **THEN** the message sent to Telegram SHALL contain the server name "claude-prod", group name "my-group", status code "500", latency "1234ms", and a UTC timestamp

#### Scenario: Alert sent to all chat IDs
- **WHEN** `telegram_chat_ids` contains ["111", "222"] and an alert is triggered
- **THEN** the system SHALL send the alert message to both chat ID "111" and chat ID "222"

#### Scenario: One chat ID fails, others succeed
- **WHEN** sending to chat ID "111" fails (Telegram API error) but "222" succeeds
- **THEN** the system SHALL log a warning for the failed delivery and continue — the cooldown key SHALL still be set
