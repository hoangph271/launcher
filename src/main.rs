#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod request_parsers;
mod responders;

use std::path::{PathBuf};
use std::env::current_dir;
use rocket_contrib::serve::{StaticFiles};
use request_parsers::RangeFromHeader;
use responders::stream_responder::{StreamResponder};

#[get("/<path..>")]
fn dirs(path: PathBuf) -> String {
    path
        .as_os_str()
        .to_string_lossy()
        .to_owned()
        .to_string()
}

fn cwd () -> PathBuf {
    current_dir().unwrap()
}

#[get("/<path..>")]
fn streams(path: PathBuf, range: RangeFromHeader) -> StreamResponder {
    StreamResponder::new(range, cwd().join(path))
}

fn main() {
    let path = current_dir().unwrap();

    rocket::ignite()
        .mount("/bin", StaticFiles::from(path))
        .mount("/streams", routes![streams])
        .mount("/dirs", routes![dirs])
        .launch();
}
