use super::super::{app_context::bins, guards::range_header::RangeFromHeader};
use rocket::http::{Header, Status};
use rocket::request::Request;
use rocket::response::{self, Redirect, Responder};
use rocket::Response;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
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
        let file_path = bins().join(self.path.to_owned());
        let file = File::open(file_path.clone());

        if let Err(_) = file {
            return Err(Status::NotFound);
        }

        let mut file = file.map_err(|_| Status::BadRequest)?;
        let file_len = file.metadata().map_err(|_| Status::BadRequest)?.len();

        let (start, end) = match self.range {
            RangeFromHeader::Nope => {
                let request_path = format!(
                    "/bins/{}",
                    self.path.to_string_lossy().to_owned().to_string()
                );
                return Redirect::to(request_path).respond_to(req);
            }
            RangeFromHeader::OpenEnd(start) => (start, file_len),
            RangeFromHeader::ClosedEnd(start, end) => (start, end),
        };

        if let Ok(_) = file.seek(SeekFrom::Start(start)) {
            let mime = mime_guess::from_path(self.path).first();
            let mut response = Response::build();

            if let Some(mime) = mime {
                response.header(Header::new(
                    "Content-Range",
                    format!("bytes {}-{}/{}", start, end, file_len),
                ));
                response.header(Header::new("Accept-Ranges", "bytes"));
                response.header(Header::new("Content-Length", (end - start).to_string()));
                response.header(Header::new("Content-Type", String::from(mime.as_ref())));
            }

            return response
                .header(Header::new("Accept-Ranges", "bytes"))
                .streamed_body(file.take(end - start))
                .ok();
        }

        Err(Status::BadRequest)
    }
}
