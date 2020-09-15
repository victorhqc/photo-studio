use crate::helpers::uuid::Uuid;
use crate::schema::{albums, photos, users};
use chrono::naive::serde::ts_seconds;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(
  Serialize, Deserialize, Debug, PartialEq, Clone, Insertable, Identifiable, Associations, Queryable,
)]
#[table_name = "users"]
pub struct User {
  pub id: Uuid,
  pub email: String,
  pub created_at: NaiveDateTime,
  #[serde(with = "ts_seconds")]
  pub updated_at: NaiveDateTime,
}

#[derive(
  Serialize, Deserialize, Debug, PartialEq, Clone, Insertable, Identifiable, Associations, Queryable,
)]
#[table_name = "albums"]
pub struct Album {
  pub id: Uuid,
  pub name: String,
  pub description: Option<String>,
  #[serde(with = "ts_seconds")]
  pub created_at: NaiveDateTime,
  #[serde(with = "ts_seconds")]
  pub updated_at: NaiveDateTime,
  pub deleted: bool,
}

#[derive(
  Serialize, Deserialize, Debug, PartialEq, Clone, Insertable, Identifiable, Associations, Queryable,
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
