## MODIFIED Requirements

### Requirement: Proxy handler emits uptime check entries
The proxy handler SHALL generate a `request_id` UUID at the start of each request. For every server attempt (including count-tokens default server attempts, connection errors, failover status codes, and TTFT timeouts), the handler SHALL emit an UptimeCheckEntry via `uptime_tx.try_send()`.

#### Scenario: Uptime entry emitted on successful response
- **WHEN** a server returns HTTP 200
- **THEN** the proxy SHALL emit an UptimeCheckEntry with the server's status_code=200 and latency_ms before returning the response

#### Scenario: Uptime entry emitted on failover
- **WHEN** a server returns a failover status code (e.g., 429) and the proxy continues to the next server
- **THEN** the proxy SHALL emit an UptimeCheckEntry with the server's status_code and latency_ms before trying the next server

#### Scenario: Uptime entry emitted on connection error
- **WHEN** a server connection fails
- **THEN** the proxy SHALL emit an UptimeCheckEntry with status_code=0 and the elapsed latency_ms

#### Scenario: Uptime entry emitted on TTFT timeout
- **WHEN** a server's first chunk exceeds the TTFT timeout
- **THEN** the proxy SHALL emit an UptimeCheckEntry with status_code=0 and the elapsed latency_ms

#### Scenario: All attempts share request_id
- **WHEN** a proxy request tries multiple servers
- **THEN** all emitted UptimeCheckEntry records SHALL share the same request_id UUID
