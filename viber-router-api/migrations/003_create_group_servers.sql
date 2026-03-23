CREATE TABLE group_servers (
    group_id UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    server_id UUID NOT NULL REFERENCES servers(id),
    priority INTEGER NOT NULL,
    model_mappings JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (group_id, server_id)
);
