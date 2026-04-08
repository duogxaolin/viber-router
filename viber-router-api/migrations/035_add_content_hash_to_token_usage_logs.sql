ALTER TABLE token_usage_logs ADD COLUMN IF NOT EXISTS content_hash TEXT;

CREATE INDEX IF NOT EXISTS idx_token_usage_logs_group_content_hash
    ON token_usage_logs (group_id, content_hash, created_at);
