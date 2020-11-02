use crate::connection::Repo;
use photo_core::models::{BookMe, ModelError, User};
use snafu::{Backtrace, ResultExt};

pub async fn update_or_create(repo: Repo, user: &User, email: String) -> Result<BookMe> {
    let user = user.clone();
    repo.run(move |conn| {
        let info = BookMe::update_or_create(&conn, &email, &user).context(Model)?;

        Ok(info)
    })
    .await
}

pub async fn find_by_user(repo: Repo, user: &User) -> Result<BookMe> {
    let user = user.clone();
    repo.run(move |conn| {
        let info = BookMe::find_by_user(&conn, &user).context(Model)?;

        Ok(info)
    })
    .await
}

pub type Result<T, E = BookMeError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum BookMeError {
    #[snafu(display("Problem with model: {}", cause))]
    Model {
        #[snafu(source)]
        cause: ModelError,
        backtrace: Backtrace,
    },
}
