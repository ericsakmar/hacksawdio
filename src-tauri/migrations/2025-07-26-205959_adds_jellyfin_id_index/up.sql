-- Adds uniqueness constraint to the `jellyfin_id` column
CREATE UNIQUE INDEX idx_albums_jellyfin_id ON albums (jellyfin_id);
