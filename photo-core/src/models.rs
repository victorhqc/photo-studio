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
    #[serde(with = "ts_seconds")]
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

#[derive(Serialize, Deserialize, Debug, Clone, AsChangeset)]
#[table_name = "albums"]
struct UpdateAlbum {
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "ts_seconds")]
    pub updated_at: NaiveDateTime,
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

    pub fn update(&self, conn: &Conn, name: String, description: Option<String>) -> Result<Album> {
        let updated = self.prepare_update(name, description);

        let album: Album = {
            use crate::schema::albums::dsl::*;

            diesel::update(albums)
                .filter(id.eq(self.id))
                .set(updated)
                .execute(conn)
                .context(Query)?;

            albums.order(updated_at.desc()).first(conn).context(Query)?
        };

        Ok(album)
    }

    pub fn delete(&self, conn: &Conn) -> Result<()> {
        use crate::schema::albums::dsl::*;

        conn.execute("PRAGMA foreign_keys = ON").context(Query)?;

        diesel::delete(albums.filter(id.eq(self.id)))
            .execute(conn)
            .context(Query)?;

        Ok(())
    }

    pub fn find_by_id(conn: &Conn, a_id: &str) -> Result<Album> {
        let album: Album = {
            use crate::schema::albums::dsl::*;

            albums.filter(id.eq(a_id)).first(conn).context(Query)?
        };

        Ok(album)
    }

    fn prepare_update(&self, name: String, description: Option<String>) -> UpdateAlbum {
        let now = Utc::now().naive_utc();

        UpdateAlbum {
            name,
            description,
            updated_at: now,
        }
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
    pub index_in_album: i32,
    pub src: String,
    pub main_color: String,
    pub description: Option<String>,
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "ts_seconds")]
    pub updated_at: NaiveDateTime,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, AsChangeset)]
#[table_name = "photos"]
struct UpdatePhoto {
    pub index_in_album: i32,
    pub description: Option<String>,
    #[serde(with = "ts_seconds")]
    pub updated_at: NaiveDateTime,
}

impl Photo {
    pub fn new(
        album: &Album,
        user: &User,
        index_in_album: i32,
        src: String,
        main_color: String,
        description: Option<String>,
    ) -> Self {
        let now = Utc::now().naive_utc();

        Self {
            id: Uuid::new_v4(),
            album_id: album.id.clone(),
            user_id: user.id.clone(),
            index_in_album,
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

    pub fn update(
        &self,
        conn: &Conn,
        index_in_album: i32,
        description: Option<String>,
    ) -> Result<Photo> {
        let updated = self.prepare_update(index_in_album, description);
        let photo: Photo = {
            use crate::schema::photos::dsl::*;

            diesel::update(photos)
                .filter(id.eq(self.id))
                .set(updated)
                .execute(conn)
                .context(Query)?;

            photos.order(updated_at.desc()).first(conn).context(Query)?
        };

        Ok(photo)
    }

    pub fn delete(&self, conn: &Conn) -> Result<()> {
        use crate::schema::photos::dsl::*;
        conn.execute("PRAGMA foreign_keys = ON").context(Query)?;

        diesel::delete(photos.filter(id.eq(self.id)))
            .execute(conn)
            .context(Query)?;

        Ok(())
    }

    fn prepare_update(&self, index_in_album: i32, description: Option<String>) -> UpdatePhoto {
        let now = Utc::now().naive_utc();

        UpdatePhoto {
            index_in_album,
            description,
            updated_at: now,
        }
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
