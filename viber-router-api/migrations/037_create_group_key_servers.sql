-- Junction table for per-key server assignments
CREATE TABLE group_key_servers (
    group_key_id UUID NOT NULL REFERENCES group_keys(id) ON DELETE CASCADE,
    server_id UUID NOT NULL REFERENCES servers(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (group_key_id, server_id)
);

CREATE INDEX idx_group_key_servers_server_id ON group_key_servers(server_id);
