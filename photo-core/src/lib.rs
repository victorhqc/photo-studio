#[cfg(target_os = "linux")]
extern crate openssl;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate snafu_derive;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

pub mod connection;
pub mod custom_migrations;
pub mod helpers;
pub mod models;
pub mod schema;
