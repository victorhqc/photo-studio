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

pub async fn update(
    repo: Repo,
    photo: &Photo,
    index_in_album: i32,
    description: Option<String>,
) -> Result<Photo> {
    let photo = photo.clone();
    repo.run(move |conn| {
        let photo = photo
            .update(&conn, index_in_album, description)
            .context(Model)?;

        Ok(photo)
    })
    .await
}

pub async fn find_by_id(repo: Repo, id: String) -> Result<Photo> {
    repo.run(move |conn| {
        let photo = Photo::find_by_id(&conn, &id).context(Model)?;

        Ok(photo)
    })
    .await
}

pub type Result<T, E = PhotoError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum PhotoError {
    #[snafu(display("Problem with model: {}", cause))]
    Model {
        #[snafu(source)]
        cause: ModelError,
        backtrace: Backtrace,
    },
}
