ALTER TABLE proxy_logs
    ADD COLUMN IF NOT EXISTS request_body JSONB,
    ADD COLUMN IF NOT EXISTS request_headers JSONB,
    ADD COLUMN IF NOT EXISTS upstream_url TEXT;
