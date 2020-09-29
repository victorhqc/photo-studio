use crate::connection::Repo;
use photo_core::models::{Album, AlbumWithPhotos, ModelError, Photo, User};
use snafu::{Backtrace, ResultExt};

pub async fn create(
    repo: Repo,
    user: &User,
    name: String,
    description: Option<String>,
) -> Result<Album> {
    let user = user.clone();
    repo.run(move |conn| {
        let album = Album::new(&user, name, description);

        let album = album.insert(&conn).context(Model)?;

        Ok(album)
    })
    .await
}

pub async fn update(
    repo: Repo,
    album: &Album,
    name: String,
    description: Option<String>,
) -> Result<Album> {
    let album = album.clone();
    repo.run(move |conn| {
        let album = album.update(&conn, name, description).context(Model)?;
        Ok(album)
    })
    .await
}

pub async fn delete(repo: Repo, album: &Album) -> Result<()> {
    let album = album.clone();
    repo.run(move |conn| {
        album.delete(&conn).context(Model)?;

        Ok(())
    })
    .await
}

pub async fn find_by_id(repo: Repo, id: String) -> Result<Album> {
    repo.run(move |conn| {
        let album = Album::find_by_id(&conn, &id).context(Model)?;

        Ok(album)
    })
    .await
}

pub async fn find_all(repo: Repo, user: &User) -> Result<Vec<AlbumWithPhotos>> {
    let user = user.clone();
    repo.run(move |conn| {
        let albums = Album::find_all(&conn, &user).context(Model)?;

        Ok(albums)
    })
    .await
}

pub async fn find_main_public(repo: Repo, user: &User) -> Result<AlbumWithPhotos> {
    let user = user.clone();
    repo.run(move |conn| {
        let album = Album::find_main_public(&conn, &user).context(Model)?;

        Ok(album)
    })
    .await
}

pub async fn photos(repo: Repo, id: String) -> Result<Vec<Photo>> {
    repo.run(move |conn| {
        let album = Album::find_by_id(&conn, &id).context(Model)?;
        let photos = album.photos(&conn).context(Model)?;

        Ok(photos)
    })
    .await
}

pub type Result<T, E = AlbumError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum AlbumError {
    #[snafu(display("Problem with model: {}", cause))]
    Model {
        #[snafu(source)]
        cause: ModelError,
        backtrace: Backtrace,
    },
}
