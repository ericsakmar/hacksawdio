-- adds track_index to the track table
ALTER TABLE tracks
ADD COLUMN track_index INTEGER NOT NULL DEFAULT 0;
