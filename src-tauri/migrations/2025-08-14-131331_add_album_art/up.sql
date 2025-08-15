-- Adds image_id and image_path to the albums table
ALTER TABLE albums ADD COLUMN image_id TEXT;
ALTER TABLE albums ADD COLUMN image_path TEXT;
