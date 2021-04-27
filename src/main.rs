#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod app_context;
mod guards;
mod libs;
mod routers;

use app_context::{bins, init_app};
use nanoid::nanoid;
use rocket::{http::Status, Response};
use rocket_contrib::serve::StaticFiles;
use routers::{dirs::DirsResponder, streams};
use std::path::PathBuf;

#[get("/<path..>")]
fn dirs(path: PathBuf) -> DirsResponder {
    DirsResponder::new(path)
}
#[get("/")]
fn dirs_index() -> DirsResponder {
    dirs(PathBuf::from(""))
}

#[catch(404)]
fn not_found<'r>() -> Response<'r> {
    Response::build().status(Status::ImATeapot).finalize()
}
use self::diesel::prelude::*;
use self::libs::models::{User, UserData};
use self::libs::schema::users;
use self::libs::schema::users::dsl::*;
use rocket_contrib::json::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, AsChangeset)]
#[table_name = "users"]
struct NewUser {
    email: String,
    nickname: String,
}
#[post("/", data = "<new_user>")]
fn create_user<'r>(new_user: Json<NewUser>) -> Status {
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
fn get_user<'a>(user_id: String) -> JsonValue {
    let conn = libs::establish_connection();

    let user = users.find(user_id).first::<User>(&conn).unwrap();
    json!(user)
}
#[get("/")]
fn get_users<'a>() -> JsonValue {
    let conn = libs::establish_connection();

    let all_users = users.load::<User>(&conn).unwrap();
    json!(all_users)
}
#[put("/<user_id>", data = "<user>")]
fn update_user(user_id: String, user: Json<NewUser>) {
    let conn = libs::establish_connection();

    diesel::update(users.find(&user_id))
        .set(user.into_inner())
        .execute(&conn)
        .unwrap();
}
#[delete("/<user_id>")]
fn del_user(user_id: String) {
    let conn = libs::establish_connection();

    diesel::delete(users.find(user_id)).execute(&conn).unwrap();
}

fn main() {
    init_app();

    rocket::ignite()
        .mount("/bins", StaticFiles::from(bins()))
        .mount(
            "/users",
            routes![get_user, get_users, create_user, del_user, update_user],
        )
        .mount(
            "/streams",
            routes![streams::stream_down, streams::stream_up],
        )
        .mount("/dirs", routes![dirs, dirs_index])
        .register(catchers![not_found])
        .launch();
}
