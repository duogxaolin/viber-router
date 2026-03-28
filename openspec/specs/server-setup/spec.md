## Requirements

### Requirement: Configuration loaded from .env file
The system SHALL load environment variables from a `.env` file at startup using dotenvy. If the `.env` file does not exist, the system SHALL continue using existing environment variables without error. The following optional environment variable is added: LOG_RETENTION_DAYS (default: 30).

#### Scenario: .env file exists
- **WHEN** a `.env` file exists in the working directory
- **THEN** its variables are loaded into the environment before config parsing

#### Scenario: .env file missing
- **WHEN** no `.env` file exists
- **THEN** the server starts normally using existing environment variables

#### Scenario: LOG_RETENTION_DAYS configured
- **WHEN** LOG_RETENTION_DAYS is set to 7
- **THEN** the system SHALL use 7 days as the log retention period

#### Scenario: LOG_RETENTION_DAYS not set
- **WHEN** LOG_RETENTION_DAYS is not set
- **THEN** the system SHALL default to 30 days for log retention

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
