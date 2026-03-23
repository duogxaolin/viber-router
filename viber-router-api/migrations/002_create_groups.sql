CREATE TABLE groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    api_key TEXT NOT NULL UNIQUE,
    failover_status_codes JSONB NOT NULL DEFAULT '[429,500,502,503]',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_groups_api_key ON groups (api_key);
CREATE INDEX idx_groups_name ON groups (name);
CREATE INDEX idx_groups_is_active ON groups (is_active);
