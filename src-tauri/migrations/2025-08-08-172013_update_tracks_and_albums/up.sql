-- Recreate the 'albums' table without the 'downloaded' column.
CREATE TABLE albums_recreated (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    jellyfin_id VARCHAR(255) NOT NULL,
    title VARCHAR NOT NULL,
    artist VARCHAR NOT NULL,
    path TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO albums_recreated (id, jellyfin_id, title, artist, path, created_at, updated_at)
SELECT id, jellyfin_id, title, artist, path, COALESCE(created_at, CURRENT_TIMESTAMP), COALESCE(updated_at, CURRENT_TIMESTAMP) FROM albums;

DROP TABLE albums;

ALTER TABLE albums_recreated RENAME TO albums;

-- Recreate the 'tracks' table to add ON DELETE CASCADE and remove the 'downloaded' column.
CREATE TABLE tracks_recreated (
  id INTEGER PRIMARY KEY NOT NULL,
  jellyfin_id TEXT UNIQUE NOT NULL,
  name TEXT NOT NULL,
  album_id INTEGER NOT NULL REFERENCES albums(id) ON DELETE CASCADE,
  path TEXT,
  track_index INTEGER,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO tracks_recreated (id, jellyfin_id, name, album_id, path, track_index)
SELECT id, jellyfin_id, name, album_id, path, track_index FROM tracks;

DROP TABLE tracks;

ALTER TABLE tracks_recreated RENAME TO tracks;
