#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

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


#[get("/")]
fn users<'a>() -> rocket_contrib::json::JsonValue {
    use self::diesel::dsl::sql;
    use self::libs::models::*;
    use self::diesel::prelude::*;
    use self::libs::schema::users;
    use self::libs::schema::users::dsl::*;

    let conn = libs::establish_connection();
    let new_id = nanoid!();

    // ! DEL all users
    diesel::delete(users.filter(sql("1 = 1")))
        .execute(&conn)
        .unwrap();

    // ! CREATE new user
    let user = NewUser {
        id: new_id.to_string(),
        email: "hoangph271@gmail.com",
        nickname: "@Me...!",
    };

    diesel::insert_into(users::table)
        .values(&user)
        .execute(&conn)
        .expect("Insert user failed...!");

    // ? READ all users
    let all_users: Vec<User> = users.limit(1).load::<User>(&conn).expect("");
    let user = all_users.get(0).unwrap();

    // ? UPDATE first user
    diesel::update(users.find(&user.id))
        .set(nickname.eq("me"))
        .execute(&conn)
        .unwrap();

    // ? READ & print all users again
    let all_users: Vec<User> = users.limit(1).load::<User>(&conn).expect("");

    // use serde_json;
    // serde_json::to_string(&all_users).unwrap()
    use rocket_contrib::json;
    json!(all_users)
}

fn main() {
    init_app();
    users();

    rocket::ignite()
        .mount("/bins", StaticFiles::from(bins()))
        .mount("/users", routes![users])
        .mount(
            "/streams",
            routes![streams::stream_down, streams::stream_up],
        )
        .mount("/dirs", routes![dirs, dirs_index])
        .register(catchers![not_found])
        .launch();
}
