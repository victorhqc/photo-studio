#[cfg(target_family = "linux")]
extern crate openssl;

#[macro_use]
extern crate diesel;

pub mod helpers;
pub mod models;
mod schema;
