use crate::connection::Conn;
use crate::helpers::uuid::Uuid;
use crate::models::User;
use chrono::Utc;
use diesel::prelude::*;
use snafu::{ResultExt, Snafu};

impl User {
  pub fn new(email: String) -> Self {
    let now = Utc::now().naive_utc();

    Self {
      id: Uuid::new_v4(),
      email,
      created_at: now,
      updated_at: now,
    }
  }

  pub fn insert(&self, conn: &Conn) -> Result<User> {
    let user: User = {
      use crate::schema::users::dsl::*;

      diesel::insert_into(users)
        .values(self)
        .execute(conn)
        .context(Query)?;

      users.order(created_at.desc()).first(conn).context(Query)?
    };

    Ok(user)
  }
}

pub type Result<T, E = UserError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum UserError {
  #[snafu(display("Could not get SQLite connection: {}", source))]
  GetConnection { source: diesel::r2d2::PoolError },

  #[snafu(display("Query Failed: {}", source))]
  Query { source: diesel::result::Error },
}
