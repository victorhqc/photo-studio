use crate::connection::Conn;
use crate::helpers::uuid::Uuid;
use crate::schema::{albums, book_me, photos, users};
use chrono::naive::serde::ts_seconds;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::prelude::*;
// use serde::{Deserialize, Serialize};
use snafu::ResultExt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Insertable, Identifiable, Queryable)]
#[diesel(table_name = users)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub picture: Option<String>,
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "ts_seconds")]
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn new(email: String, picture: Option<String>) -> Self {
        let now = Utc::now().naive_utc();

        Self {
            id: Uuid::new_v4(),
            email,
            picture,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn insert(&self, conn: &mut Conn) -> Result<User> {
        let user: User = {
            use crate::schema::users::dsl::*;

            diesel::insert_into(users)
                .values(self)
                .execute(conn)
                .context(QuerySnafu)?;

            users
                .order(created_at.desc())
                .first(conn)
                .context(QuerySnafu)?
        };

        Ok(user)
    }

    pub fn find_by_email(conn: &mut Conn, u_email: &str) -> Result<User> {
        use crate::schema::users::dsl::*;

        let user = users
            .filter(email.eq(u_email))
            .first(conn)
            .context(QuerySnafu)?;

        Ok(user)
    }

    pub fn find_by_id(conn: &mut Conn, u_id: &str) -> Result<User> {
        use crate::schema::users::dsl::*;

        let user = users.filter(id.eq(u_id)).first(conn).context(QuerySnafu)?;

        Ok(user)
    }

    pub fn find_all(conn: &mut Conn) -> Result<Vec<User>> {
        let users: Vec<User> = users::table.load(conn).context(QuerySnafu)?;

        Ok(users)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Insertable, Identifiable, Queryable)]
#[diesel(table_name = albums)]
#[serde(rename_all = "camelCase")]
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
#[diesel(table_name = albums)]
#[serde(rename_all = "camelCase")]
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

    pub fn insert(&self, conn: &mut Conn) -> Result<Album> {
        let album: Album = {
            use crate::schema::albums::dsl::*;

            diesel::insert_into(albums)
                .values(self)
                .execute(conn)
                .context(QuerySnafu)?;

            albums
                .order(created_at.desc())
                .first(conn)
                .context(QuerySnafu)?
        };

        Ok(album)
    }

    pub fn update(
        &self,
        conn: &mut Conn,
        name: String,
        description: Option<String>,
    ) -> Result<Album> {
        let updated = self.prepare_update(name, description);

        let album: Album = {
            use crate::schema::albums::dsl::*;

            diesel::update(albums)
                .filter(id.eq(self.id))
                .set(updated)
                .execute(conn)
                .context(QuerySnafu)?;

            albums
                .order(updated_at.desc())
                .first(conn)
                .context(QuerySnafu)?
        };

        Ok(album)
    }

    pub fn delete(&self, conn: &mut Conn) -> Result<()> {
        use crate::schema::albums::dsl::*;

        diesel::sql_query("PRAGMA foreign_keys = ON")
            .execute(conn)
            .context(QuerySnafu)?;

        diesel::delete(albums.filter(id.eq(self.id)))
            .execute(conn)
            .context(QuerySnafu)?;

        Ok(())
    }

    pub fn find_by_id(conn: &mut Conn, a_id: &str) -> Result<Album> {
        let album: Album = {
            use crate::schema::albums::dsl::*;

            albums.filter(id.eq(a_id)).first(conn).context(QuerySnafu)?
        };

        Ok(album)
    }

    pub fn find_by_name(conn: &mut Conn, user: &User, a_name: &str) -> Result<AlbumWithPhotos> {
        let album: Album = {
            use crate::schema::albums::dsl::*;

            albums
                .filter(user_id.eq(user.id))
                .filter(name.eq(a_name))
                .first(conn)
                .context(QuerySnafu)?
        };

        let photos = Photo::belonging_to(&album)
            .load::<Photo>(conn)
            .context(QuerySnafu)?;

        Ok((album, photos))
    }

    /// Find all albums including the first picture of each album.
    pub fn find_all(conn: &mut Conn, user: &User) -> Result<Vec<AlbumWithPhotos>> {
        let albums: Vec<Album> = {
            use crate::schema::albums::dsl::*;

            albums
                .filter(deleted.eq(false))
                .filter(user_id.eq(user.id))
                .load::<Album>(conn)
                .context(QuerySnafu)?
        };

        let photos: Vec<Vec<Photo>> = albums
            .iter()
            .map(|album| {
                let photos = Photo::belonging_to(album)
                    .limit(1)
                    .load::<Photo>(conn)
                    .unwrap();

                photos
            })
            .collect();

        let data = albums.into_iter().zip(photos).collect::<Vec<_>>();

        Ok(data)
    }

    pub fn find_main_public(conn: &mut Conn, user: &User) -> Result<AlbumWithPhotos> {
        // TODO: Implement public & main album functionality. For now it'll return the first album.
        let album: Album = {
            use crate::schema::albums::dsl::*;

            albums
                .filter(deleted.eq(false))
                .filter(user_id.eq(user.id))
                .first(conn)
                .context(QuerySnafu)?
        };

        let photos = Photo::belonging_to(&album)
            .load::<Photo>(conn)
            .context(QuerySnafu)?;

        Ok((album, photos))
    }

    pub fn photos(&self, conn: &mut Conn) -> Result<Vec<Photo>> {
        use crate::schema::photos::dsl::*;

        let results: Vec<Photo> = photos
            .filter(deleted.eq(false))
            .filter(album_id.eq(self.id))
            .load::<Photo>(conn)
            .context(QuerySnafu)?;

        Ok(results)
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
#[diesel(table_name = photos, belongs_to(Album), belongs_to(User))]
#[serde(rename_all = "camelCase")]
pub struct Photo {
    pub id: Uuid,
    pub album_id: Uuid,
    pub user_id: Uuid,
    pub index_in_album: i32,
    pub s3_id: String,
    pub src: String,
    pub main_color: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub width: i32,
    pub height: i32,
    pub is_favorite: bool,
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "ts_seconds")]
    pub updated_at: NaiveDateTime,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, AsChangeset)]
#[diesel(table_name = photos)]
struct UpdatePhoto {
    pub index_in_album: i32,
    pub is_favorite: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(with = "ts_seconds")]
    pub updated_at: NaiveDateTime,
}

impl Photo {
    pub fn new(
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
    ) -> Self {
        let now = Utc::now().naive_utc();

        Self {
            id: Uuid::new_v4(),
            album_id: album.id.clone(),
            user_id: user.id.clone(),
            index_in_album,
            s3_id,
            src: src.clone(),
            main_color,
            title,
            description,
            width,
            height,
            is_favorite,
            created_at: now,
            updated_at: now,
            deleted: false,
        }
    }

    pub fn insert(&self, conn: &mut Conn) -> Result<Photo> {
        let photo: Photo = {
            use crate::schema::photos::dsl::*;

            diesel::insert_into(photos)
                .values(self)
                .execute(conn)
                .context(QuerySnafu)?;

            photos
                .order(created_at.desc())
                .first(conn)
                .context(QuerySnafu)?
        };

        Ok(photo)
    }

    pub fn update(
        &self,
        conn: &mut Conn,
        index_in_album: i32,
        is_favorite: bool,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<Photo> {
        let updated = self.prepare_update(index_in_album, is_favorite, title, description);
        let photo: Photo = {
            use crate::schema::photos::dsl::*;

            diesel::update(photos)
                .filter(id.eq(self.id))
                .set(updated)
                .execute(conn)
                .context(QuerySnafu)?;

            photos
                .order(updated_at.desc())
                .first(conn)
                .context(QuerySnafu)?
        };

        Ok(photo)
    }

    pub fn delete(&self, conn: &mut Conn) -> Result<()> {
        use crate::schema::photos::dsl::*;

        diesel::sql_query("PRAGMA foreign_keys = ON")
            .execute(conn)
            .context(QuerySnafu)?;

        diesel::delete(photos.filter(id.eq(self.id)))
            .execute(conn)
            .context(QuerySnafu)?;

        Ok(())
    }

    pub fn find_by_id(conn: &mut Conn, p_id: &str) -> Result<Photo> {
        use crate::schema::photos::dsl::*;

        let photo = photos.filter(id.eq(p_id)).first(conn).context(QuerySnafu)?;

        Ok(photo)
    }

    fn prepare_update(
        &self,
        index_in_album: i32,
        is_favorite: bool,
        title: Option<String>,
        description: Option<String>,
    ) -> UpdatePhoto {
        let now = Utc::now().naive_utc();

        UpdatePhoto {
            index_in_album,
            is_favorite,
            title,
            description,
            updated_at: now,
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    AsChangeset,
    Insertable,
    Identifiable,
    Associations,
    Queryable,
)]
#[diesel(table_name = book_me, belongs_to(User))]
#[serde(rename_all = "camelCase")]
pub struct BookMe {
    id: Uuid,
    user_id: Uuid,
    pub email: String,
}

impl BookMe {
    pub fn new(email: String, user: &User) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: user.id,
            email,
        }
    }

    pub fn find_by_user(conn: &mut Conn, user: &User) -> Result<BookMe> {
        use crate::schema::book_me::dsl::*;

        let info = book_me
            .filter(user_id.eq(user.id))
            .first(conn)
            .context(QuerySnafu)?;

        Ok(info)
    }

    pub fn update_or_create(conn: &mut Conn, book_email: &str, user: &User) -> Result<BookMe> {
        let existing: Result<BookMe> = BookMe::find_by_user(conn, user);

        let book_me_info: BookMe = match existing {
            Ok(book_me_info) => {
                use crate::schema::book_me::dsl::*;

                diesel::update(book_me)
                    .filter(id.eq(book_me_info.id))
                    .set(email.eq(book_email))
                    .execute(conn)
                    .context(QuerySnafu)?;

                book_me
                    .filter(id.eq(book_me_info.id))
                    .first(conn)
                    .context(QuerySnafu)?
            }
            Err(_) => {
                use crate::schema::book_me::dsl::*;

                let info = BookMe::new(String::from(book_email), user);

                diesel::insert_into(book_me)
                    .values(info)
                    .execute(conn)
                    .context(QuerySnafu)?;

                book_me
                    .filter(user_id.eq(user.id))
                    .first(conn)
                    .context(QuerySnafu)?
            }
        };

        Ok(book_me_info)
    }
}

pub type Result<T, E = ModelError> = std::result::Result<T, E>;

pub type AlbumWithPhotos = (Album, Vec<Photo>);

#[derive(Debug, Snafu)]
pub enum ModelError {
    #[snafu(display("Could not get SQLite connection: {}", source))]
    GetConnection { source: diesel::r2d2::PoolError },

    #[snafu(display("Query Failed: {}", source))]
    Query { source: diesel::result::Error },
}
