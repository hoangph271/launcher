pub mod models;
pub mod responders;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").unwrap();

    SqliteConnection::establish(&database_url).unwrap()
}
