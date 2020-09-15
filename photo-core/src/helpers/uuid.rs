// Implementation based on:
// https://github.com/vorleser/vorleser-server/blob/master/src/helpers/uuid.rs

use diesel::backend::Backend;
use diesel::deserialize;
use diesel::serialize;
use diesel::sql_types::{Text, VarChar};
use diesel::sqlite::Sqlite;
use diesel::types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::str::FromStr;
use uuid;

pub type ParseError = uuid::Error;

#[derive(
  Debug, Hash, Eq, PartialEq, Serialize, Deserialize, Clone, Copy, AsExpression, FromSqlRow,
)]
#[sql_type = "Text"]
pub struct Uuid(uuid::Uuid);

impl Uuid {
  pub fn parse_str(input: &str) -> Result<Self, ParseError> {
    uuid::Uuid::parse_str(input).map(Uuid)
  }

  pub fn new_v4() -> Self {
    Uuid(uuid::Uuid::new_v4())
  }

  pub fn hyphenated(&self) -> uuid::adapter::Hyphenated {
    self.0.to_hyphenated()
  }
}

impl std::fmt::Display for Uuid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.to_string())
  }
}

impl ToSql<Text, Sqlite> for Uuid {
  fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, Sqlite>) -> serialize::Result {
    let hyphenated = self.0.to_hyphenated().to_string();
    ToSql::<VarChar, Sqlite>::to_sql(&hyphenated, out)
  }
}

impl FromSql<VarChar, Sqlite> for Uuid {
  fn from_sql(value: Option<&<Sqlite as Backend>::RawValue>) -> deserialize::Result<Self> {
    let text: String = FromSql::<Text, Sqlite>::from_sql(value)?;

    match uuid::Uuid::from_str(&text) {
      Err(_) => Err("Can't parse UUID datatype".into()),
      Ok(v) => Ok(Uuid(v)),
    }
  }
}
