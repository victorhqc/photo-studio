use crate::connection::{connect, Conn};
use crate::helpers::uuid::Uuid;
use crate::models::{Album, ModelError, User};
use crate::schema::custom_migrations;
use chrono::naive::serde::ts_seconds;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use snafu::{Backtrace, ResultExt};

lazy_static! {
    static ref MIGRATIONS: Vec<String> = [String::from("lifestyle_album"),].to_vec();
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
            "Lifestyle".to_string(),
            Some("Family & Lifestyle".to_string()),
        );

        album.insert(&conn).unwrap();
    });

    let migration = CustomMigration::new("lifestyle_album".to_string());
    migration.insert(&conn)?;

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
