-- brings back the download flag
ALTER TABLE tracks
ADD COLUMN downloaded BOOLEAN DEFAULT FALSE;

ALTER TABLE albums
ADD COLUMN downloaded BOOLEAN DEFAULT FALSE;
