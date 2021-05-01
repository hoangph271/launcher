use super::super::diesel::prelude::*;
use super::super::libs;
use super::super::libs::models::{User, UserData};
use super::super::libs::responders::EZRespond;
use super::super::libs::schema::users;
use super::super::libs::schema::users::dsl::*;
use nanoid::nanoid;
use rocket::http::Status;
use rocket_contrib::json::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub nickname: String,
}

#[post("/", data = "<new_user>")]
pub fn post_user<'r>(new_user: Json<NewUser>) -> EZRespond<'r> {
    let conn = libs::establish_connection();
    let email_existed =
        diesel::select(diesel::dsl::exists(users.filter(email.eq(&new_user.email))))
            .get_result(&conn)
            .unwrap();

    if email_existed {
        return EZRespond::by_status(Status::Conflict);
    }

    let user = UserData {
        id: &nanoid!(),
        email: &new_user.email,
        nickname: &new_user.nickname,
    };

    diesel::insert_into(users::table)
        .values(&user)
        .execute(&conn)
        .expect("Insert user failed...!");

    EZRespond::by_status(Status::Created)
}

#[get("/<user_id>")]
pub fn get_user<'a>(user_id: String) -> EZRespond<'a> {
    let conn = libs::establish_connection();

    if let Ok(user) = users.find(user_id).first::<User>(&conn) {
        EZRespond::json(json!(user), None)
    } else {
        EZRespond::by_status(Status::NotFound)
    }
}

#[get("/")]
pub fn get_users<'a>() -> EZRespond<'a> {
    let conn = libs::establish_connection();

    if let Ok(all_users) = users.load::<User>(&conn) {
        EZRespond::json(json!(all_users), None)
    } else {
        EZRespond::by_status(Status::InternalServerError)
    }
}

#[put("/<user_id>", data = "<user>")]
pub fn update_user<'a>(user_id: String, user: Json<NewUser>) -> EZRespond<'a> {
    let conn = libs::establish_connection();

    let rows_count = diesel::update(users.find(&user_id))
        .set(user.into_inner())
        .execute(&conn);

    EZRespond::by_db_changed(rows_count)
}

#[delete("/<user_id>")]
pub fn delete_user<'a>(user_id: String) -> EZRespond<'a> {
    let conn = libs::establish_connection();
    let rows_count = diesel::delete(users.find(user_id)).execute(&conn);

    EZRespond::by_db_changed(rows_count)
}
