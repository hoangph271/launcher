use super::super::constants::response_messsage;
use rocket::http::{ContentType, Header, Status};
use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket::Response;
use rocket_contrib::json::JsonValue;
use std::io;
use std::io::Cursor;

trait SizeBody: io::Read + io::Seek {}
pub enum Body {
    // Empty,
    Text(String),
    Json(JsonValue),
}
pub struct EZRespond<'r> {
    pub status: Option<Status>,
    pub content_type: Option<ContentType>,
    pub header: Option<Vec<Header<'r>>>,
    pub body: Body,
}

impl<'a> EZRespond<'a> {
    pub fn by_status<'r>(status: Status) -> EZRespond<'r> {
        let body = match status {
            Status::Ok => Body::Text(String::from(response_messsage::OK)),
            Status::Created => Body::Text(String::from(response_messsage::CREATED)),
            Status::Conflict => Body::Text(String::from(response_messsage::CONFLICT)),
            Status::NotFound => Body::Text(String::from(response_messsage::NOT_FOUND)),
            Status::InternalServerError => {
                Body::Text(String::from(response_messsage::INTERNAL_SERVER_ERROR))
            }
            Status::ImATeapot => Body::Text(String::from(response_messsage::IM_A_TEAPOT)),
            Status::Unauthorized => Body::Text(String::from(response_messsage::UNAUTHORIZED)),
            Status::UnprocessableEntity => {
                Body::Text(String::from(response_messsage::UNPROCESSABLE_ENTITY))
            }
            status => Body::Text(format!("{}", status)),
        };

        EZRespond {
            body,
            content_type: None,
            status: Some(status),
            header: None,
        }
    }

    pub fn text<'r>(text: String, status: Option<Status>) -> EZRespond<'r> {
        let status = status.unwrap_or(Status::Ok);

        EZRespond {
            body: Body::Text(text),
            content_type: Some(ContentType::Plain),
            header: None,
            status: Some(status),
        }
    }

    pub fn json<'r>(json: JsonValue, status: Option<Status>) -> EZRespond<'r> {
        let status = status.unwrap_or(Status::Ok);

        EZRespond {
            body: Body::Json(json),
            content_type: Some(ContentType::JSON),
            header: None,
            status: Some(status),
        }
    }

    #[allow(dead_code)]
    pub fn by_db_ok<'r, T>(db_result: Result<T, diesel::result::Error>) -> EZRespond<'r> {
        match db_result {
            Ok(_) => EZRespond::by_status(Status::Ok),
            Err(e) => {
                dbg!(e);
                EZRespond::by_status(Status::InternalServerError)
            }
        }
    }

    pub fn by_db_changed<'r>(db_result: Result<usize, diesel::result::Error>) -> EZRespond<'r> {
        match db_result {
            Ok(rows_count) => {
                if rows_count == 0 {
                    EZRespond::by_status(Status::NotFound)
                } else {
                    EZRespond::by_status(Status::Ok)
                }
            }
            Err(e) => {
                dbg!(e);
                EZRespond::by_status(Status::InternalServerError)
            }
        }
    }
}

impl<'a> Responder<'a> for EZRespond<'a> {
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        let status = self.status.unwrap_or(Status::Ok);

        let mut response = Response::build();
        let response = response.status(status);

        if let Some(content_type) = self.content_type {
            response.header(content_type);
        }

        for header in self.header.unwrap_or_else(|| vec![]) {
            response.header(header);
        }

        match self.body {
            Body::Text(text) => {
                response.sized_body(Cursor::new(text));
            }
            Body::Json(json) => {
                response.sized_body(Cursor::new(json.to_string()));
            } // Body::Empty => {}
        }

        response.finalize().respond_to(req)
    }
}
