use crate::connection::Repo;
use photo_core::models::{Album, ModelError, Photo, User};
use snafu::{Backtrace, ResultExt};

pub async fn create(
    repo: Repo,
    album: &Album,
    user: &User,
    index_in_album: i32,
    s3_id: String,
    src: String,
    main_color: String,
    title: Option<String>,
    description: Option<String>,
    width: i32,
    height: i32,
    is_favorite: bool,
) -> Result<Photo> {
    let album = album.clone();
    let user = user.clone();
    repo.run(move |conn| {
        let photo = Photo::new(
            &album,
            &user,
            index_in_album,
            s3_id,
            src,
            main_color,
            title,
            description,
            width,
            height,
            is_favorite,
        );
        let photo = photo.insert(&conn).context(Model)?;

        Ok(photo)
    })
    .await
}

pub async fn update(
    repo: Repo,
    photo: &Photo,
    index_in_album: i32,
    is_favorite: bool,
    title: Option<String>,
    description: Option<String>,
) -> Result<Photo> {
    let photo = photo.clone();
    repo.run(move |conn| {
        let photo = photo
            .update(&conn, index_in_album, is_favorite, title, description)
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

pub async fn delete(repo: Repo, photo: &Photo) -> Result<()> {
    let photo = photo.clone();

    repo.run(move |conn| {
        photo.delete(&conn).context(Model)?;

        Ok(())
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
