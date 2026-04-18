ALTER TABLE settings ADD COLUMN ct_always_estimate BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE settings ADD COLUMN ct_anthropic_base_url TEXT;
ALTER TABLE settings ADD COLUMN ct_anthropic_api_key TEXT;
