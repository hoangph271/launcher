use super::super::request_parsers::RangeFromHeader;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, NamedFile, Responder, Stream};
use rocket::Response;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Cursor, SeekFrom};
use std::path::PathBuf;

pub struct StreamResponder {
    range: RangeFromHeader,
    path: PathBuf,
}
impl StreamResponder {
    pub fn new(range: RangeFromHeader, path: PathBuf) -> Self {
        StreamResponder { range, path }
    }
}

impl<'a> Responder<'a> for StreamResponder {
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        let file = File::open(self.path.clone());

        if let Err(_) = file {
            let message = format!("{:?}", self.path);
            return Response::build()
                .status(Status::NotFound)
                .header(ContentType::Plain)
                .sized_body(Cursor::new(message))
                .ok();
        }

        let mut file = file.unwrap();

        if let RangeFromHeader::OpenEnd(start) = self.range {
            if let Ok(_) = file.seek(SeekFrom::Start(start as u64)) {
                return Stream::from(file).respond_to(req);
            }
        }

        if let RangeFromHeader::ClosedEnd(start, end) = self.range {
            if let Ok(_) = file.seek(SeekFrom::Start(start as u64)) {
                return Stream::from(file.take(end as u64)).respond_to(req);
            }
        }

        return NamedFile::open(self.path).unwrap().respond_to(req); // FIXME:
    }
}
