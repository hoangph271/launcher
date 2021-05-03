use super::super::models::{User, UserData};
use super::super::schema::users;
use super::super::schema::users::dsl::*;
use super::execute_auto_connect;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

pub fn email_existed(email_to_find: &str, conn: Option<&SqliteConnection>) -> Result<bool, Error> {
    execute_auto_connect(conn, |conn| {
        diesel::select(diesel::dsl::exists(
            users.filter(users::email.eq(&email_to_find)),
        ))
        .first::<bool>(conn)
    })
}

pub fn create(user: UserData, conn: Option<&SqliteConnection>) -> Result<usize, Error> {
    execute_auto_connect(conn, |conn| {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
    })
}

pub fn find_by_id(user_id: &str, conn: Option<&SqliteConnection>) -> Result<User, Error> {
    execute_auto_connect(conn, |conn| users.find(user_id).first::<User>(conn))
}

pub fn find_by_email(email_to_find: &str, conn: Option<&SqliteConnection>) -> Result<User, Error> {
    execute_auto_connect(conn, |conn| {
        users.filter(email.eq(email_to_find)).first::<User>(conn)
    })
}

pub fn find(conn: Option<&SqliteConnection>) -> Result<Vec<User>, Error> {
    execute_auto_connect(conn, |conn| users.load::<User>(conn))
}

pub fn delete_by_id(user_id: &str, conn: Option<&SqliteConnection>) -> Result<usize, Error> {
    execute_auto_connect(conn, |conn| {
        diesel::delete(users.find(user_id)).execute(conn)
    })
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UpdatePayload {
    pub email: String,
    pub name: String,
}

pub fn update(
    user_id: &str,
    update_payload: UpdatePayload,
    conn: Option<&SqliteConnection>,
) -> Result<usize, Error> {
    execute_auto_connect(conn, move |conn| {
        diesel::update(users.find(&user_id))
            .set(update_payload)
            .execute(conn)
    })
}
