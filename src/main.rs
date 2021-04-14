#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::path::{PathBuf};
use std::convert::Infallible;
use rocket::{Outcome, Response};
use rocket::request;
use rocket::request::{Request, FromRequest};
use rocket_contrib::serve::{StaticFiles};
use rocket::http::{Status, ContentType};
use rocket::response::{self, Responder, Stream, NamedFile};
use std::io::prelude::*;
use std::fs::File;
use std::io::{SeekFrom, Cursor};
use std::env::current_dir;

struct StreamResponser {
    range: Range,
    path: PathBuf
}
impl<'a> Responder<'a> for StreamResponser {
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        let cwd_path = current_dir().unwrap();
        let path = cwd_path.join("bin").join(self.path);
        let file = File::open(path.clone());

        if let Err(_) = file {
            let message = format!("{:?}", path);
            return Response::build()
                .status(Status::NotFound)
                .header(ContentType::Plain)
                .sized_body(Cursor::new(message))
                .ok();
        }

        let mut file = file.unwrap();

        if let Range::OpenEnd(start) = self.range {
            if let Ok(_) = file.seek(SeekFrom::Start(start as u64)) {
                return Stream::from(file).respond_to(req);
            }
        }

        if let Range::ClosedEnd(start, end) = self.range {
            if let Ok(_) = file.seek(SeekFrom::Start(start as u64)) {
                return Stream::from(file.take(end as u64)).respond_to(req);
            }
        }

        return NamedFile::open(path)
            .unwrap()
            .respond_to(req); // FIXME:
    }
}

#[derive(Debug)]
enum Range {
    Nope,
    OpenEnd(usize),
    ClosedEnd(usize, usize)
}
impl<'a, 'r> FromRequest<'a, 'r> for Range {
    type Error = Infallible;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let range_header = request.headers().get_one("range");

        if let None = range_header {
            return Outcome::Success(Range::Nope);
        }

        let range = &(range_header.unwrap())["bytes=".len()..];
        let ranges: Vec<Result<usize, core::num::ParseIntError>> = String::from(range)
            .split("-")
            .map(|s| str::parse::<usize>(s))
            .collect();

        if let Ok(start) = ranges[0] {
            if ranges.len() < 2 {
                return Outcome::Success(Range::OpenEnd(start));
            }

            if let Ok(end) = ranges[1] {
                Outcome::Success(Range::ClosedEnd(start, end))
            } else {
                Outcome::Success(Range::OpenEnd(start))
            }
        } else {
            Outcome::Success(Range::Nope)
        }
    }
}

#[get("/<path..>")]
fn dirs(path: PathBuf) -> String {
    path
        .as_os_str()
        .to_string_lossy()
        .to_owned()
        .to_string()
}

#[get("/<path..>")]
fn streams(path: PathBuf, range: Range) -> StreamResponser {
    StreamResponser {
        path,
        range
    }
}

fn main() {
    let path = current_dir().unwrap();

    rocket::ignite()
        .mount("/bin", StaticFiles::from(path))
        .mount("/streams", routes![streams])
        .mount("/dirs", routes![dirs])
        .launch();
}
