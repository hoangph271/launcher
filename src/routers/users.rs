use super::super::diesel::prelude::*;
use super::super::libs;
use super::super::libs::models::{User, UserData};
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
pub fn post_user<'r>(new_user: Json<NewUser>) -> Status {
    let conn = libs::establish_connection();
    let email_existed =
        diesel::select(diesel::dsl::exists(users.filter(email.eq(&new_user.email))))
            .get_result(&conn)
            .unwrap();

    if email_existed {
        // TODO: This...? 409
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

    Status::Ok
}

#[get("/<user_id>")]
pub fn get_user(user_id: String) -> JsonValue {
    let conn = libs::establish_connection();

    let user = users.find(user_id).first::<User>(&conn).unwrap();

    json!(user)
}

#[get("/")]
pub fn get_users<'a>() -> JsonValue {
    let conn = libs::establish_connection();

    let all_users = users.load::<User>(&conn).unwrap();
    json!(all_users)
}

#[put("/<user_id>", data = "<user>")]
pub fn update_user(user_id: String, user: Json<NewUser>) {
    let conn = libs::establish_connection();

    diesel::update(users.find(&user_id))
        .set(user.into_inner())
        .execute(&conn)
        .unwrap();
}

#[delete("/<user_id>")]
pub fn delete_user(user_id: String) {
    let conn = libs::establish_connection();

    diesel::delete(users.find(user_id)).execute(&conn).unwrap();
}
