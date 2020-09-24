use crate::auth::Profile;
use crate::connection::Repo;
use photo_core::models::{ModelError, User};
use snafu::{Backtrace, ResultExt};

pub async fn find_or_create<T: Profile>(repo: Repo, profile: T) -> Result<User> {
    let new_user = profile.new_user();

    repo.run(
        move |conn| match User::find_by_email(&conn, &new_user.email).context(Model) {
            Ok(u) => Ok(u),
            Err(_) => {
                let user = new_user.insert(&conn).unwrap();

                Ok(user)
            }
        },
    )
    .await
}

pub async fn find_by_email(repo: Repo, u_email: String) -> Result<User> {
    repo.run(move |conn| {
        let user = User::find_by_email(&conn, &u_email).context(Model)?;

        Ok(user)
    })
    .await
}

pub type Result<T, E = UserError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum UserError {
    #[snafu(display("Problem with model: {}", cause))]
    Model {
        #[snafu(source)]
        cause: ModelError,
        backtrace: Backtrace,
    },
}
