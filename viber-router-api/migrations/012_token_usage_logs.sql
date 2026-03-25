CREATE TABLE IF NOT EXISTS token_usage_logs (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    group_id UUID NOT NULL,
    server_id UUID NOT NULL,
    model TEXT,
    input_tokens INTEGER NOT NULL,
    output_tokens INTEGER NOT NULL,
    cache_creation_tokens INTEGER,
    cache_read_tokens INTEGER,
    is_dynamic_key BOOLEAN NOT NULL DEFAULT false,
    key_hash TEXT,
    PRIMARY KEY (id, created_at)
) PARTITION BY RANGE (created_at);

CREATE INDEX IF NOT EXISTS idx_token_usage_logs_group_created
    ON token_usage_logs (group_id, created_at);

CREATE INDEX IF NOT EXISTS idx_token_usage_logs_key_hash_created
    ON token_usage_logs (key_hash, created_at);
