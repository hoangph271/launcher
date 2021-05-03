use super::super::models::{Auth, AuthData};
use super::super::schema::auths;
use super::super::schema::auths::dsl::*;
use super::execute_auto_connect;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

pub fn create(auth_data: AuthData, conn: Option<&SqliteConnection>) -> Result<usize, Error> {
    execute_auto_connect(conn, |conn| {
        diesel::insert_into(auths::table)
            .values(&auth_data)
            .execute(conn)
    })
}

pub fn delete_by_email(
    target_email: &str,
    conn: Option<&SqliteConnection>,
) -> Result<usize, Error> {
    execute_auto_connect(conn, |conn| {
        diesel::delete(auths.filter(auths::email.eq(target_email))).execute(conn)
    })
}

pub fn find_by_email(target_email: &str, conn: Option<&SqliteConnection>) -> Result<Auth, Error> {
    execute_auto_connect(conn, |conn| {
        auths.filter(email.eq(target_email)).first::<Auth>(conn)
    })
}
