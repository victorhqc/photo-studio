use crate::auth::Profile;
use crate::connection::Repo;
use diesel::prelude::*;
use photo_core::models::{ModelError, User};
use snafu::{Backtrace, ResultExt, Snafu};

pub async fn find_or_create<T: Profile>(repo: Repo, profile: T) -> Result<User> {
    let new_user = profile.new_user();

    repo.run(move |conn| {
        let user_email = new_user.email.clone();
        let user = {
            use photo_core::schema::users::dsl::*;

            users.filter(email.eq(user_email)).first::<User>(&conn)
        };

        match user {
            Ok(u) => Ok(u),
            Err(_) => {
                let user = new_user.insert(&conn).unwrap();

                Ok(user)
            }
        }
    })
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
