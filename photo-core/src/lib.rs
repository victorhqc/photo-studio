#[cfg(target_os = "linux")]
extern crate openssl;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

pub mod connection;
pub mod helpers;
pub mod models;
pub mod schema;
