#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod app_context;
mod request_parsers;
mod responders;

use app_context::{bins, init_app};
use request_parsers::RangeFromHeader;
use responders::stream_responder::StreamResponder;
use rocket_contrib::serve::StaticFiles;
use std::path::PathBuf;

#[get("/<path..>")]
fn dirs(path: PathBuf) -> String {
    bins()
        .join(path)
        .as_os_str()
        .to_string_lossy()
        .to_owned()
        .to_string()
}
#[get("/")]
fn dirs_index() -> String {
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
