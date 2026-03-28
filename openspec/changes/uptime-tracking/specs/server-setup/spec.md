## MODIFIED Requirements

### Requirement: Application startup initializes all subsystems
The application SHALL create an mpsc channel for uptime check entries (capacity 10,000), spawn the uptime_buffer flush task, add `uptime_tx: mpsc::Sender<UptimeCheckEntry>` to AppState, and ensure partitions for the `uptime_checks` table on startup alongside existing partition management.

#### Scenario: Uptime buffer initialization
- **WHEN** the application starts
- **THEN** the system SHALL create an mpsc channel with capacity 10,000 for UptimeCheckEntry, spawn the uptime_buffer flush task, and include `uptime_tx` in AppState

#### Scenario: Partition management includes uptime_checks
- **WHEN** the application starts or the daily maintenance job runs
- **THEN** the system SHALL ensure partitions and drop expired partitions for `uptime_checks` alongside `proxy_logs`, `ttft_logs`, and `token_usage_logs`

#### Scenario: Graceful shutdown drains uptime buffer
- **WHEN** the application receives a shutdown signal
- **THEN** the system SHALL drop the AppState (closing the uptime_tx sender) and await the uptime flush task to drain remaining entries
