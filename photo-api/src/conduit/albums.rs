use crate::connection::Repo;
use photo_core::models::{Album, ModelError};
use snafu::{Backtrace, ResultExt, Snafu};

pub async fn create(repo: Repo, name: String, description: Option<String>) -> Result<Album> {
    repo.run(move |conn| {
        let album = Album::new(name, description);

        let album = album.insert(&conn).context(Model)?;

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
