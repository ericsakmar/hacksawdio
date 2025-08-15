use crate::db::Pool;
use crate::models::{Album, NewAlbum, NewTrack, Track};
use crate::schema::albums::dsl as albums_dsl;
use crate::schema::tracks::dsl as tracks_dsl;
use chrono::Utc;
use diesel::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database pool error: {0}")]
    DbPoolError(#[from] r2d2::Error),
    #[error("Database error: {0}")]
    DbError(#[from] diesel::result::Error),
    #[error("Generic error: {0}")]
    GenericError(String),
}

pub struct Repository {
    db_pool: Pool,
}

impl Repository {
    pub fn new(db_pool: Pool) -> Self {
        Self { db_pool }
    }

    pub fn find_album(&self, album_id: &str) -> Result<Option<Album>, RepositoryError> {
        let mut conn = self.db_pool.get()?;
        albums_dsl::albums
            .filter(albums_dsl::jellyfin_id.eq(album_id))
            .select(Album::as_select())
            .first(&mut conn)
            .optional()
            .map_err(RepositoryError::DbError)
    }

    pub fn get_recents_offline(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Album>, RepositoryError> {
        let mut conn = self.db_pool.get()?;
        albums_dsl::albums
            .order(albums_dsl::updated_at.desc())
            .limit(limit.unwrap_or(100) as i64)
            .offset(offset.unwrap_or(0) as i64)
            .select(Album::as_select())
            .load::<Album>(&mut conn)
            .map_err(RepositoryError::DbError)
    }

    pub fn search_albums_offline(
        &self,
        search: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Album>, RepositoryError> {
        let mut conn = self.db_pool.get()?;
        albums_dsl::albums
            .filter(
                albums_dsl::title
                    .like(format!("%{}%", search))
                    .or(albums_dsl::artist.like(format!("%{}%", search))),
            )
            .order(albums_dsl::title.asc())
            .limit(limit.unwrap_or(100) as i64)
            .offset(offset.unwrap_or(0) as i64)
            .select(Album::as_select())
            .load::<Album>(&mut conn)
            .map_err(RepositoryError::DbError)
    }

    pub fn get_album_details(
        &self,
        album_id: &str,
    ) -> Result<Option<(Album, Vec<Track>)>, RepositoryError> {
        let mut conn = self.db_pool.get()?;
        let album_option = albums_dsl::albums
            .filter(albums_dsl::jellyfin_id.eq(album_id))
            .select(Album::as_select())
            .first::<Album>(&mut conn)
            .optional()?;

        if let Some(album) = album_option {
            let tracks = tracks_dsl::tracks
                .filter(tracks_dsl::album_id.eq(album.id))
                .select(Track::as_select())
                .order(tracks_dsl::track_index.asc())
                .load::<Track>(&mut conn)?;
            Ok(Some((album, tracks)))
        } else {
            Ok(None)
        }
    }

    pub fn get_downloaded_album_ids(
        &self,
        album_ids: Vec<String>,
    ) -> Result<Vec<String>, RepositoryError> {
        let mut conn = self.db_pool.get()?;
        albums_dsl::albums
            .filter(albums_dsl::jellyfin_id.eq_any(album_ids))
            .select(albums_dsl::jellyfin_id)
            .load(&mut conn)
            .map_err(RepositoryError::DbError)
    }

    pub fn create_album(
        &self,
        jellyfin_id_str: &str,
        title_str: &str,
        artist_str: &str,
        image_id: Option<&str>,
    ) -> Result<Album, RepositoryError> {
        let mut conn = self.db_pool.get()?;

        let new_album = NewAlbum {
            jellyfin_id: jellyfin_id_str,
            title: title_str,
            artist: artist_str,
            image_id: image_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        diesel::insert_into(albums_dsl::albums)
            .values(&new_album)
            .execute(&mut conn)?;

        self.find_album(jellyfin_id_str)?.ok_or_else(|| {
            RepositoryError::GenericError("Album not found after insertion".to_string())
        })
    }

    pub fn insert_track(&self, new_track: &NewTrack) -> Result<(), RepositoryError> {
        let mut conn = self.db_pool.get()?;
        diesel::insert_into(tracks_dsl::tracks)
            .values(new_track)
            .execute(&mut conn)?;
        Ok(())
    }

    pub fn mark_album_as_downloaded(
        &self,
        album_id: &str,
        album_path: &str,
        image_path: Option<&str>,
    ) -> Result<(), RepositoryError> {
        let mut conn = self.db_pool.get()?;
        diesel::update(albums_dsl::albums.filter(albums_dsl::jellyfin_id.eq(album_id)))
            .set((
                albums_dsl::path.eq(album_path),
                albums_dsl::image_path.eq(image_path),
                albums_dsl::updated_at.eq(diesel::dsl::now),
            ))
            .execute(&mut conn)?;
        Ok(())
    }

    pub fn delete_album_and_tracks(&self, album: &Album) -> Result<(), RepositoryError> {
        let mut conn = self.db_pool.get()?;

        diesel::delete(tracks_dsl::tracks.filter(tracks_dsl::album_id.eq(album.id)))
            .execute(&mut conn)?;

        diesel::delete(albums_dsl::albums.filter(albums_dsl::jellyfin_id.eq(&album.jellyfin_id)))
            .execute(&mut conn)?;

        Ok(())
    }
}
