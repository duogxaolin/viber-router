ALTER TABLE settings ADD COLUMN blocked_paths TEXT[] NOT NULL DEFAULT '{}';
