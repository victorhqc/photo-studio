use crate::auth::Profile;
use crate::connection::Repo;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use photo_core::models::User;

pub async fn find_or_create<T: Profile>(repo: Repo, profile: T) -> Result<User, DieselError> {
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

pub async fn find_by_email(repo: Repo, u_email: String) -> Result<User, DieselError> {
    repo.run(move |conn| {
        use photo_core::schema::users::dsl::*;

        users.filter(email.eq(u_email)).first::<User>(&conn)
    })
    .await
}
