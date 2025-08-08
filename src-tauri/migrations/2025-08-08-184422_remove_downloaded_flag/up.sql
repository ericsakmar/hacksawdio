-- remove downloaded flag from the tracks
ALTER TABLE tracks
DROP COLUMN downloaded;

-- remove downloaded flag from the albums
ALTER TABLE albums
DROP COLUMN downloaded;
