CREATE TABLE IF NOT EXISTS uptime_checks (
    id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    group_id UUID NOT NULL,
    server_id UUID NOT NULL,
    status_code SMALLINT NOT NULL,
    latency_ms INTEGER NOT NULL,
    request_id UUID NOT NULL,
    PRIMARY KEY (id, created_at)
) PARTITION BY RANGE (created_at);

CREATE INDEX IF NOT EXISTS idx_uptime_checks_group_server_time
    ON uptime_checks (group_id, server_id, created_at);

CREATE INDEX IF NOT EXISTS idx_uptime_checks_group_time
    ON uptime_checks (group_id, created_at);
