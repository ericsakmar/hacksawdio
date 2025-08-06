-- This file should undo anything in `up.sql`
ALTER TABLE albums DROP COLUMN created_at;
ALTER TABLE albums DROP COLUMN updated_at;