## Why

The proxy currently forwards all API paths to upstream servers. Some paths (e.g., `/v1/completions` for OpenAI-style completions) are not supported or should be restricted. Admins need a global setting to block specific API paths so the proxy returns 404 immediately, preventing unnecessary upstream traffic and clearly signaling unsupported endpoints.

## What Changes

- Add a `blocked_paths` column (TEXT array) to the `settings` table
- Cache blocked paths in a dedicated Redis key (`settings:blocked_paths`) for fast proxy-time lookup
- Check blocked paths as the very first step in `proxy_handler` — before API key extraction — returning a 404 Anthropic-style JSON error for any matching path
- Matching uses exact path comparison (query strings are excluded from matching since they are not part of the URI path)
- Invalidate the Redis cache when admin updates settings
- Add a "Blocked API Paths" section to the Settings UI with chip-based path management

## Capabilities

### New Capabilities
- `blocked-api-paths`: Global blocklist of API paths that the proxy rejects with 404 before any authentication or routing logic

### Modified Capabilities
- `telegram-alert-settings`: The settings model and API gain a new `blocked_paths` field alongside existing telegram fields
- `config-cache`: A new Redis key `settings:blocked_paths` is added for proxy-time blocked path lookup, invalidated on settings update

## Impact

- **Database**: Migration to add `blocked_paths TEXT[] DEFAULT '{}'` to `settings` table
- **Backend**: `Settings` model, settings route handlers, proxy handler, cache module
- **Frontend**: `SettingsPage.vue` — new UI section
- **API**: `GET/PUT /api/admin/settings` request/response gains `blocked_paths` field
