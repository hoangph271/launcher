#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod app_context;
mod guards;
mod routers;

use app_context::{bins, init_app};
use rocket_contrib::serve::StaticFiles;
use routers::{dirs::DirsResponder, streams};
use rocket::{Response, http::Status};
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
    Response::build()
        .status(Status::ImATeapot)
        .finalize()
}

fn main() {
    init_app();

    rocket::ignite()
        .mount("/bins", StaticFiles::from(bins()))
        .mount(
            "/streams",
            routes![streams::stream_down, streams::stream_up],
        )
        .mount("/dirs", routes![dirs, dirs_index])
        .register(catchers![not_found])
        .launch();
}
