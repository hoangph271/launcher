use super::establish_connection;
use diesel::sqlite::SqliteConnection;

pub mod auths_service;
pub mod users_service;

pub fn execute_auto_connect<T>(
    conn: Option<&SqliteConnection>,
    execute: impl FnOnce(&SqliteConnection) -> T,
) -> T {
    if let Some(conn) = conn {
        execute(conn)
    } else {
        execute(&establish_connection())
    }
}
