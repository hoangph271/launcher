use super::establish_connection;
use super::models::{Auth, AuthData};
use super::schema::auths;
use super::schema::auths::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

pub fn create(auth_data: AuthData, conn: Option<&SqliteConnection>) -> Result<usize, Error> {
    let execute = |conn| {
        diesel::insert_into(auths::table)
            .values(&auth_data)
            .execute(conn)
    };

    if let Some(conn) = conn {
        execute(conn)
    } else {
        execute(&establish_connection())
    }
}

pub fn delete_by_email(
    target_email: &str,
    conn: Option<&SqliteConnection>,
) -> Result<usize, Error> {
    let execute = |conn| diesel::delete(auths.filter(auths::email.eq(target_email))).execute(conn);

    if let Some(conn) = conn {
        execute(conn)
    } else {
        execute(&establish_connection())
    }
}

pub fn find_by_email(target_email: &str, conn: Option<&SqliteConnection>) -> Result<Auth, Error> {
    let execute = |conn| auths.filter(email.eq(target_email)).first::<Auth>(conn);

    if let Some(conn) = conn {
        execute(conn)
    } else {
        execute(&establish_connection())
    }
}
