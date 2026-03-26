ALTER TABLE token_usage_logs ADD COLUMN IF NOT EXISTS group_key_id UUID;

CREATE INDEX IF NOT EXISTS idx_token_usage_logs_group_key_created
    ON token_usage_logs (group_key_id, created_at);
