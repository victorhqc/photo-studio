use gotham_middleware_diesel::{self};
use photo_core::connection::{get_database_url, Conn};
use std::env;

pub type Repo = gotham_middleware_diesel::Repo<Conn>;

pub fn repo() -> Repo {
    let url = match env::var("DATABASE_URL") {
        Ok(u) => Some(u),
        Err(_) => None,
    };
    let database_url = get_database_url(url);

    Repo::new(&database_url)
}
