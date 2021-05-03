use super::super::super::libs::responders::EZRespond;
use rocket::http::Status;

pub fn not_found<'r>() -> EZRespond<'r> {
    EZRespond::by_status(Status::ImATeapot)
}
