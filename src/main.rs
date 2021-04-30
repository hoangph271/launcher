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
mod constants;

use app_context::{bins, init_app};
use rocket_contrib::serve::StaticFiles;
use routers::{dirs, streams, users, others};

fn main() {
    init_app();

    rocket::ignite()
        .mount("/bins", StaticFiles::from(bins()))
        .mount(
            "/users",
            routes![
                users::get_user,
                users::get_users,
                users::post_user,
                users::delete_user,
                users::update_user
            ],
        )
        .mount(
            "/streams",
            routes![streams::stream_down, streams::stream_up],
        )
        .mount("/dirs", routes![dirs::get_dir, dirs::get_index_dir])
        .mount("/status", routes![others::status])
        .register(catchers![others::not_found, others::unauthorized])
        .launch();
}
