use crate::connection::Repo;
use photo_core::models::{Album, ModelError, Photo, User};
use snafu::{Backtrace, ResultExt, Snafu};

pub async fn create(
    repo: Repo,
    album: &Album,
    user: &User,
    index_in_album: i32,
    src: String,
    main_color: String,
    description: Option<String>,
) -> Result<Photo> {
    let album = album.clone();
    let user = user.clone();
    repo.run(move |conn| {
        let photo = Photo::new(&album, &user, index_in_album, src, main_color, description);
        let photo = photo.insert(&conn).context(Model)?;

        Ok(photo)
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
