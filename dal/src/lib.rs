#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

pub mod auths_service;
pub mod models;
pub mod schema;
pub mod users_service;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").unwrap();

    SqliteConnection::establish(&database_url).unwrap()
}
