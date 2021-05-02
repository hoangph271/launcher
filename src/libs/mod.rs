pub mod models;
pub mod responders;
pub mod schema;

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").unwrap();

    SqliteConnection::establish(&database_url).unwrap()
}
