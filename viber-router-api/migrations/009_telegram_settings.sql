-- Single-row settings table for Telegram alert configuration.
-- Enforced single row via CHECK constraint (id = 1) and PRIMARY KEY DEFAULT 1.
CREATE TABLE settings (
    id INT PRIMARY KEY DEFAULT 1,
    telegram_bot_token TEXT,
    telegram_chat_ids TEXT[] NOT NULL DEFAULT '{}',
    alert_status_codes INT[] NOT NULL DEFAULT '{500,502,503}',
    alert_cooldown_mins INT NOT NULL DEFAULT 5,
    CONSTRAINT settings_single_row CHECK (id = 1)
);
