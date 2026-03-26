CREATE TABLE IF NOT EXISTS group_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    api_key TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    monthly_token_limit BIGINT,
    monthly_request_limit BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_group_keys_api_key ON group_keys (api_key);
CREATE INDEX IF NOT EXISTS idx_group_keys_group_id ON group_keys (group_id);
