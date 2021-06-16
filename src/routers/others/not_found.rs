use crate::libs::responders::EZRespond;
use rocket::{http::Status, Request};

pub fn not_found<'r>(req: &Request) -> EZRespond<'r> {
    let white_list = ["/bins/"];

    let is_in_white_list = white_list.iter().any(|&route| {
        let path = req.uri().path();
        path.starts_with(&route)
    });

    let status = if is_in_white_list {
        Status::NotFound
    } else {
        Status::ImATeapot
    };

    EZRespond::by_status(status)
}
