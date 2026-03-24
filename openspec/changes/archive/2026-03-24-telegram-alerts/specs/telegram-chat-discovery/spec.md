## ADDED Requirements

### Requirement: Get Telegram chats API
The system SHALL provide GET `/api/admin/settings/telegram-chats` (admin-authenticated) that calls the Telegram `getUpdates` API with `limit=100` using the configured bot token and returns a deduplicated list of chats that have messaged the bot.

#### Scenario: Chats found
- **WHEN** an authenticated admin calls GET `/api/admin/settings/telegram-chats` and the bot has received messages
- **THEN** the system SHALL return HTTP 200 with a list of unique chats, each containing `chat_id` (string), `first_name` (string, nullable), and `username` (string, nullable)

#### Scenario: No updates available
- **WHEN** an authenticated admin calls GET `/api/admin/settings/telegram-chats` and `getUpdates` returns an empty result
- **THEN** the system SHALL return HTTP 200 with an empty list `{"chats": []}`

#### Scenario: Bot token not configured
- **WHEN** an authenticated admin calls GET `/api/admin/settings/telegram-chats` and `telegram_bot_token` is NULL
- **THEN** the system SHALL return HTTP 400 with `{"error": "Bot token not configured"}`

#### Scenario: Telegram API returns error
- **WHEN** the Telegram `getUpdates` call returns a non-2xx response (e.g., 401 invalid token)
- **THEN** the system SHALL return HTTP 502 with `{"error": "<telegram error description>"}`

### Requirement: Admin UI chat discovery
The Admin UI Settings page SHALL provide a "Get Chat IDs from bot" button that fetches available chats and presents them in a selectable list. The user SHALL be able to select chats and add their IDs to the `telegram_chat_ids` configuration.

#### Scenario: Fetch and display chats
- **WHEN** the user clicks "Get Chat IDs from bot"
- **THEN** the UI SHALL call GET `/api/admin/settings/telegram-chats`, show a loading state, then display a list of chats with `first_name`, `username` (if available), and `chat_id`

#### Scenario: User selects chats to add
- **WHEN** the user ticks one or more chats in the list and clicks "Add Selected"
- **THEN** the selected `chat_id` values SHALL be appended to the `telegram_chat_ids` field (deduplicating any already present)

#### Scenario: No chats found
- **WHEN** GET `/api/admin/settings/telegram-chats` returns an empty list
- **THEN** the UI SHALL display the message "No chats found. Send a message to your bot first."

#### Scenario: API error during fetch
- **WHEN** GET `/api/admin/settings/telegram-chats` returns an error
- **THEN** the UI SHALL display the error message inline
