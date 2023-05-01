// Implementation based on:
// https://github.com/vorleser/vorleser-server/blob/master/src/helpers/uuid.rs

use diesel::{
    backend::RawValue,
    deserialize::{self, FromSql},
    serialize::{self, ToSql},
    sql_types::{Text, VarChar},
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid;

pub type ParseError = uuid::Error;

#[derive(
    Debug, Hash, Eq, PartialEq, Serialize, Deserialize, Clone, Copy, AsExpression, FromSqlRow,
)]
#[diesel(sql_type = Text)]
pub struct Uuid(uuid::Uuid);

impl Uuid {
    pub fn parse_str(input: &str) -> Result<Self, ParseError> {
        uuid::Uuid::parse_str(input).map(Uuid)
    }

    pub fn new_v4() -> Self {
        Uuid(uuid::Uuid::new_v4())
    }

    pub fn hyphenated(&self) -> uuid::fmt::Hyphenated {
        self.0.hyphenated()
    }
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl ToSql<Text, Sqlite> for Uuid {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        let hyphenated = self.0.hyphenated().to_string();

        out.set_value(hyphenated);
        Ok(serialize::IsNull::No)
    }
}

impl FromSql<VarChar, Sqlite> for Uuid {
    fn from_sql(value: RawValue<'_, Sqlite>) -> deserialize::Result<Self> {
        let text: String = FromSql::<Text, Sqlite>::from_sql(value)?;

        match uuid::Uuid::from_str(&text) {
            Err(_) => Err("Can't parse UUID datatype".into()),
            Ok(v) => Ok(Uuid(v)),
        }
    }
}
