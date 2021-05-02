use super::super::constants::auth_type;
use super::super::diesel::prelude::*;
use dal::{establish_connection, models::{AuthData, User, UserData}};
use super::super::libs::responders::EZRespond;
use dal::schema::auths::dsl::*;
use dal::schema::users::dsl::*;
use dal::schema::{auths, users};
use anyhow::Error;
use nanoid::nanoid;
use rocket::http::Status;
use rocket_contrib::json::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUser {
    pub email: String,
    pub nickname: String,
    pub password: String,
}

#[derive(Deserialize, Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserPayload {
    pub email: String,
    pub nickname: String,
}

#[post("/", data = "<new_user>")]
pub fn post_user<'r>(new_user: Json<NewUser>) -> EZRespond<'r> {
    let conn = establish_connection();
    let email_existed = diesel::select(diesel::dsl::exists(
        users.filter(users::email.eq(&new_user.email)),
    ))
    .get_result(&conn)
    .unwrap();

    if email_existed {
        return EZRespond::by_status(Status::Conflict);
    }

    let transaction = conn.transaction::<_, Error, _>(|| {
        let user = UserData {
            id: &nanoid!(),
            email: &new_user.email,
            nickname: &new_user.nickname,
        };

        diesel::insert_into(users::table)
            .values(&user)
            .execute(&conn)?;

        let bcrypt_password_hash =
            bcrypt::hash(new_user.password.as_bytes(), bcrypt::DEFAULT_COST)?;

        let auth = AuthData {
            id: &nanoid!(),
            auth_type: auth_type::BASIC,
            email: user.email,
            password_hash: &bcrypt_password_hash,
        };

        diesel::insert_into(auths::table)
            .values(&auth)
            .execute(&conn)?;

        Ok(())
    });

    if let Err(code) = transaction {
        dbg!(code);
        EZRespond::by_status(Status::InternalServerError)
    } else {
        EZRespond::by_status(Status::Created)
    }
}

#[get("/<user_id>")]
pub fn get_user<'a>(user_id: String) -> EZRespond<'a> {
    let conn = establish_connection();

    if let Ok(user) = users.find(user_id).first::<User>(&conn) {
        EZRespond::json(json!(user), None)
    } else {
        EZRespond::by_status(Status::NotFound)
    }
}

#[get("/")]
pub fn get_users<'a>() -> EZRespond<'a> {
    let conn = establish_connection();

    match users.load::<User>(&conn) {
        Ok(all_users) => EZRespond::json(json!(all_users), None),
        Err(e) => {
            dbg!(e);
            EZRespond::by_status(Status::InternalServerError)
        }
    }
}

#[put("/<user_id>", data = "<user>")]
pub fn update_user<'a>(user_id: String, user: Json<UserPayload>) -> EZRespond<'a> {
    let conn = establish_connection();

    let rows_count = diesel::update(users.find(&user_id))
        .set(user.into_inner())
        .execute(&conn);

    EZRespond::by_db_changed(rows_count)
}

#[delete("/<user_id>")]
pub fn delete_user<'a>(user_id: String) -> EZRespond<'a> {
    let conn = establish_connection();

    let transaction = conn.transaction::<_, Error, _>(|| {
        let user = users
            .find(user_id.to_owned())
            .first::<User>(&conn)
            .optional()?;

        if let Some(user) = user {
            diesel::delete(auths.filter(auths::email.eq(user.email))).execute(&conn)?;
            diesel::delete(users.find(user_id)).execute(&conn)?;
        }

        Ok(())
    });

    if transaction.is_ok() {
        EZRespond::by_status(Status::Ok)
    } else {
        dbg!(transaction);
        EZRespond::by_status(Status::InternalServerError)
    }
}
