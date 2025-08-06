-- Your SQL goes here
ALTER TABLE albums ADD COLUMN created_at TIMESTAMP;
ALTER TABLE albums ADD COLUMN updated_at TIMESTAMP;
UPDATE albums SET created_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP;