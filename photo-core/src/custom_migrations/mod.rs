use crate::connection::{connect, Conn};
use crate::helpers::uuid::Uuid;
use crate::models::{Album, ModelError, User};
use crate::schema::custom_migrations;
use crate::schema::photos;
use chrono::naive::serde::ts_seconds;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use snafu::{Backtrace, ResultExt};

lazy_static! {
    static ref MIGRATIONS: Vec<String> = [
        String::from("lifestyle_album"),
        String::from("image_metadata")
    ]
    .to_vec();
}

pub fn apply_custom_migrations(url: Option<String>) -> Result<()> {
    let conn = connect(url).expect("Could not get connection");

    let applied_migrations = CustomMigration::get_all(&conn)?;

    let to_apply: Vec<String> = MIGRATIONS
        .iter()
        .filter(|name| {
            let applied = applied_migrations.iter().find(|mig| &&mig.name == name);

            match applied {
                Some(_) => false,
                None => true,
            }
        })
        .map(|name| name.clone())
        .collect();

    debug!("Migrations to apply {:?}", to_apply);

    to_apply.iter().for_each(|name| {
        match &name[..] {
            "lifestyle_album" => migrate_lifestyle_album(&conn).unwrap(),
            "image_metadata" => migrate_image_metadata(&conn).unwrap(),
            _ => {}
        };
    });

    Ok(())
}

fn migrate_lifestyle_album(conn: &Conn) -> Result<()> {
    debug!("Migrating lifestyle_album");

    let users = User::find_all(&conn).context(Model)?;

    users.iter().for_each(|user| {
        let album = Album::new(
            &user,
            "lifestyle".to_string(),
            Some("Family & Lifestyle".to_string()),
        );

        album.insert(&conn).unwrap();
    });

    let migration = CustomMigration::new("lifestyle_album".to_string());
    migration.insert(&conn)?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone, AsChangeset)]
#[table_name = "photos"]
struct UpdatePhoto {
    pub width: i32,
    pub height: i32,
}

fn migrate_image_metadata(conn: &Conn) -> Result<()> {
    debug!("Migrating image_metadata");

    let users = User::find_all(&conn).context(Model)?;

    users.iter().for_each(|user| {
        let albums = Album::find_all(conn, &user).unwrap();
        albums.iter().for_each(|(album, _)| {
            let photos = album.photos(conn).unwrap();

            photos.iter().for_each(|photo| {
                let img_bytes = reqwest::blocking::get(&photo.src).unwrap().bytes().unwrap();
                let image = image::load_from_memory(&img_bytes).unwrap().to_bgr();
                let (img_width, img_height) = image.dimensions();

                {
                    use crate::schema::photos::dsl::*;

                    let updated = UpdatePhoto {
                        width: img_width as i32,
                        height: img_height as i32,
                    };

                    debug!("Updating image dimensions {:?}", updated);

                    diesel::update(photos)
                        .filter(id.eq(photo.id))
                        .set(updated)
                        .execute(conn)
                        .unwrap();
                };
            });
        });
    });

    let migration = CustomMigration::new("image_metadata".to_string());
    migration.insert(&conn)?;
    debug!("Migration ended!");

    Ok(())
}

#[derive(
    Debug,
    PartialEq,
    Clone,
    Insertable,
    Identifiable,
    Associations,
    Queryable,
    Serialize,
    Deserialize,
)]
#[table_name = "custom_migrations"]
#[serde(rename_all = "camelCase")]
pub struct CustomMigration {
    pub id: Uuid,
    pub name: String,
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
}

impl CustomMigration {
    pub fn new(name: String) -> Self {
        let now = Utc::now().naive_utc();

        CustomMigration {
            id: Uuid::new_v4(),
            name,
            created_at: now,
        }
    }

    pub fn insert(&self, conn: &Conn) -> Result<CustomMigration> {
        let migration: CustomMigration = {
            use crate::schema::custom_migrations::dsl::*;

            diesel::insert_into(custom_migrations)
                .values(self)
                .execute(conn)
                .context(Query)?;

            custom_migrations
                .order(created_at.desc())
                .first(conn)
                .context(Query)?
        };

        Ok(migration)
    }

    pub fn get_all(conn: &Conn) -> Result<Vec<CustomMigration>> {
        let migrations: Vec<CustomMigration> =
            custom_migrations::table.load(conn).context(Query)?;

        Ok(migrations)
    }
}

pub type Result<T, E = CustomMigrationError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum CustomMigrationError {
    #[snafu(display("Could not get SQLite connection: {}", source))]
    GetConnection { source: diesel::r2d2::PoolError },

    #[snafu(display("Query Failed: {}", source))]
    Query { source: diesel::result::Error },

    #[snafu(display("Problem with model: {}", cause))]
    Model {
        #[snafu(source)]
        cause: ModelError,
        backtrace: Backtrace,
    },
}
