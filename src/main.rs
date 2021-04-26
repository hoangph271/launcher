#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod app_context;
mod guards;
mod routers;

use app_context::{bins, init_app};
use guards::range_header;
use rocket_contrib::serve::StaticFiles;
use routers::{dirs::DirsResponder, streams::StreamResponder};
use std::path::PathBuf;

#[get("/<path..>")]
fn dirs(path: PathBuf) -> DirsResponder {
    DirsResponder::new(path)
}
#[get("/")]
fn dirs_index() -> DirsResponder {
    dirs(PathBuf::from(""))
}

#[get("/<path..>")]
fn streams(path: PathBuf, range: range_header::RangeFromHeader) -> StreamResponder {
    StreamResponder::new(range, path)
}

#[catch(404)]
fn not_found<'r>() -> rocket::Response<'r> {
    rocket::Response::build()
        .status(rocket::http::Status::raw(418))
        .finalize()
}

fn main() {
    init_app();

    rocket::ignite()
        .mount("/bins", StaticFiles::from(bins()))
        .mount("/streams", routes![streams])
        .mount("/dirs", routes![dirs, dirs_index])
        .register(catchers![not_found])
        .launch();
}
