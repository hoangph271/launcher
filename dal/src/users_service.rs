use super::establish_connection;
use super::models::{User, UserData};
use super::schema::users;
use super::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

pub fn email_existed(email_to_find: &str, conn: Option<&SqliteConnection>) -> Result<bool, Error> {
    let execute = |conn| {
        diesel::select(diesel::dsl::exists(
            users.filter(users::email.eq(&email_to_find)),
        ))
        .first::<bool>(conn)
    };

    if let Some(conn) = conn {
        execute(conn)
    } else {
        execute(&establish_connection())
    }
}

pub fn create(user: UserData, conn: Option<&SqliteConnection>) -> Result<usize, Error> {
    let execute = |conn| {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
    };

    if let Some(conn) = conn {
        execute(conn)
    } else {
        execute(&establish_connection())
    }
}

pub fn find_by_id(user_id: &str, conn: Option<&SqliteConnection>) -> Result<User, Error> {
    if let Some(conn) = conn {
        users.find(user_id).first::<User>(conn)
    } else {
        users.find(user_id).first::<User>(&establish_connection())
    }
}

pub fn find(conn: Option<&SqliteConnection>) -> Result<Vec<User>, Error> {
    if let Some(conn) = conn {
        users.load::<User>(conn)
    } else {
        users.load::<User>(&establish_connection())
    }
}

pub fn delete_by_id(user_id: &str, conn: Option<&SqliteConnection>) -> Result<usize, Error> {
    if let Some(conn) = conn {
        diesel::delete(users.find(user_id)).execute(conn)
    } else {
        diesel::delete(users.find(user_id)).execute(&establish_connection())
    }
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UpdatePayload {
    pub email: String,
    pub nickname: String,
}

pub fn update(
    user_id: &str,
    update_payload: UpdatePayload,
    conn: Option<&SqliteConnection>,
) -> Result<usize, Error> {
    let execute = |conn| {
        diesel::update(users.find(&user_id))
            .set(update_payload)
            .execute(conn)
    };

    if let Some(conn) = conn {
        execute(conn)
    } else {
        execute(&establish_connection())
    }
}
