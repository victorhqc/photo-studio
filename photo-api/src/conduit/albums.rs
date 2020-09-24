use crate::connection::Repo;
use photo_core::models::{Album, ModelError, User};
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
