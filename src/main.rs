#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod app_context;
mod request_parsers;
mod routers;

use app_context::{bins, init_app};
use request_parsers::RangeFromHeader;
use rocket_contrib::serve::StaticFiles;
use std::path::PathBuf;
use routers::{streams::StreamResponder, dirs::{DirsResponder}};

#[get("/<path..>")]
fn dirs (path: PathBuf) -> DirsResponder {
    DirsResponder::new(path)
}
#[get("/")]
fn dirs_index() -> DirsResponder {
    dirs(PathBuf::from(""))
}

#[get("/<path..>")]
fn streams(path: PathBuf, range: RangeFromHeader) -> StreamResponder {
    StreamResponder::new(range, path)
}

fn main() {
    init_app();

    rocket::ignite()
        .mount("/bins", StaticFiles::from(bins()))
        .mount("/streams", routes![streams])
        .mount("/dirs", routes![dirs, dirs_index])
        .launch();
}
