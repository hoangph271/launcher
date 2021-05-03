use super::super::constants::auth_type;
use super::super::libs::{json_catcher, responders::EZRespond};
use anyhow::Error;
use dal::models::{AuthData, UserData};
use dal::{auths_service, users_service};
use diesel::prelude::*;
use nanoid::nanoid;
use rocket::http::Status;
use rocket_contrib::json::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUser {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[post("/", data = "<new_user>")]
pub fn post_user<'r>(new_user: Result<Json<NewUser>, JsonError>) -> EZRespond<'r> {
    json_catcher::with_json_catcher(new_user, |new_user| {
        let email_existed = users_service::email_existed(&new_user.email, None)
            .expect("DB error on users_service::email_existed()");

        if email_existed {
            return EZRespond::by_status(Status::Conflict);
        }

        let conn = dal::establish_connection();
        let transaction = conn.transaction::<_, Error, _>(|| {
            users_service::create(
                UserData {
                    id: &nanoid!(),
                    email: &new_user.email,
                    name: &new_user.name,
                },
                Some(&conn),
            )?;

            let bcrypt_password_hash =
                bcrypt::hash(new_user.password.as_bytes(), bcrypt::DEFAULT_COST)?;

            auths_service::create(
                AuthData {
                    id: &nanoid!(),
                    auth_type: auth_type::BASIC,
                    email: &new_user.email,
                    password_hash: &bcrypt_password_hash,
                },
                Some(&conn),
            )?;

            Ok(())
        });

        if let Err(code) = transaction {
            dbg!(code);
            EZRespond::by_status(Status::InternalServerError)
        } else {
            EZRespond::by_status(Status::Created)
        }
    })
}

#[get("/<user_id>")]
pub fn get_user<'a>(user_id: String) -> EZRespond<'a> {
    if let Ok(user) = users_service::find_by_id(&user_id, None) {
        EZRespond::json(json!(user), None)
    } else {
        EZRespond::by_status(Status::NotFound)
    }
}

#[get("/")]
pub fn get_users<'a>() -> EZRespond<'a> {
    match users_service::find(None) {
        Ok(all_users) => EZRespond::json(json!(all_users), None),
        Err(e) => {
            dbg!(e);
            EZRespond::by_status(Status::InternalServerError)
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct UserPayload {
    pub email: String,
    pub name: String,
}

#[put("/<user_id>", data = "<user>")]
pub fn update_user<'a>(
    user_id: String,
    user: Result<Json<UserPayload>, JsonError>,
) -> EZRespond<'a> {
    json_catcher::with_json_catcher(user, |user| {
        let user = user.into_inner();

        let rows_count = users_service::update(
            &user_id,
            users_service::UpdatePayload {
                email: user.email,
                name: user.name,
            },
            None,
        );

        EZRespond::by_db_changed(rows_count)
    })
}

#[delete("/<user_id>")]
pub fn delete_user<'a>(user_id: String) -> EZRespond<'a> {
    let conn = dal::establish_connection();
    let transaction = conn.transaction::<_, Error, _>(|| {
        let user = users_service::find_by_id(&user_id, Some(&conn)).optional()?;

        if let Some(user) = user {
            auths_service::delete_by_email(&user.email, Some(&conn))?;
            users_service::delete_by_id(&user_id, Some(&conn))?;
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
