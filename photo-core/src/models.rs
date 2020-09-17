use crate::connection::Conn;
use crate::helpers::uuid::Uuid;
use crate::schema::{albums, photos, users};
use chrono::naive::serde::ts_seconds;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Clone,
    Insertable,
    Identifiable,
    Associations,
    Queryable,
)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub created_at: NaiveDateTime,
    #[serde(with = "ts_seconds")]
    pub updated_at: NaiveDateTime,
}

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

    pub fn find_by_email(conn: &Conn, u_email: &str) -> Result<User> {
        use crate::schema::users::dsl::*;

        let user = users.filter(email.eq(u_email)).first(conn).context(Query)?;

        Ok(user)
    }
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Clone,
    Insertable,
    Identifiable,
    Associations,
    Queryable,
)]
#[table_name = "albums"]
pub struct Album {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "ts_seconds")]
    pub updated_at: NaiveDateTime,
    pub deleted: bool,
}

impl Album {
    pub fn new(user: &User, name: String, description: Option<String>) -> Self {
        let now = Utc::now().naive_utc();

        Self {
            id: Uuid::new_v4(),
            user_id: user.id.clone(),
            name,
            description,
            created_at: now,
            updated_at: now,
            deleted: false,
        }
    }

    pub fn insert(&self, conn: &Conn) -> Result<Album> {
        let album: Album = {
            use crate::schema::albums::dsl::*;

            diesel::insert_into(albums)
                .values(self)
                .execute(conn)
                .context(Query)?;

            albums.order(created_at.desc()).first(conn).context(Query)?
        };

        Ok(album)
    }

    pub fn find_by_id(conn: &Conn, a_id: &str) -> Result<Album> {
        let album: Album = {
            use crate::schema::albums::dsl::*;

            albums.filter(id.eq(a_id)).first(conn).context(Query)?
        };

        Ok(album)
    }
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Clone,
    Insertable,
    Identifiable,
    Associations,
    Queryable,
)]
#[table_name = "photos"]
#[belongs_to(Album)]
#[belongs_to(User)]
pub struct Photo {
    pub id: Uuid,
    pub album_id: Uuid,
    pub user_id: Uuid,
    pub src: String,
    pub main_color: String,
    pub description: Option<String>,
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "ts_seconds")]
    pub updated_at: NaiveDateTime,
    pub deleted: bool,
}

impl Photo {
    pub fn new(
        album: &Album,
        user: &User,
        src: String,
        main_color: String,
        description: Option<String>,
    ) -> Self {
        let now = Utc::now().naive_utc();

        Self {
            id: Uuid::new_v4(),
            album_id: album.id.clone(),
            user_id: user.id.clone(),
            src: src.clone(),
            main_color,
            description,
            created_at: now,
            updated_at: now,
            deleted: false,
        }
    }

    pub fn insert(&self, conn: &Conn) -> Result<Photo> {
        let photo: Photo = {
            use crate::schema::photos::dsl::*;

            diesel::insert_into(photos)
                .values(self)
                .execute(conn)
                .context(Query)?;

            photos.order(created_at.desc()).first(conn).context(Query)?
        };

        Ok(photo)
    }
}

pub type Result<T, E = ModelError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum ModelError {
    #[snafu(display("Could not get SQLite connection: {}", source))]
    GetConnection { source: diesel::r2d2::PoolError },

    #[snafu(display("Query Failed: {}", source))]
    Query { source: diesel::result::Error },
}
