ALTER TABLE group_servers
  ADD COLUMN active_hours_start TEXT,
  ADD COLUMN active_hours_end TEXT,
  ADD COLUMN active_hours_timezone TEXT;
