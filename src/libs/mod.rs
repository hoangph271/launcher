pub mod models;
pub mod responders;
pub mod schema;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").unwrap();

    MysqlConnection::establish(&database_url).unwrap()
}
