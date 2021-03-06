use crate::libs::responders::EZRespond;
use rocket::Request;

mod not_found;
#[catch(404)]
pub fn not_found<'r>(req: &Request) -> EZRespond<'r> {
    not_found::not_found(req)
}

mod unauthorized;
#[catch(401)]
pub fn unauthorized<'r>() -> EZRespond<'r> {
    unauthorized::unauthorized()
}

mod unprocessable_entity;
#[catch(422)]
pub fn unprocessable_entity<'r>() -> EZRespond<'r> {
    EZRespond::by_status(rocket::http::Status::UnprocessableEntity)
}

mod server_status;
#[get("/")]
pub fn server_status<'r>(basic_auth: server_status::BasicAuth) -> EZRespond<'r> {
    server_status::server_status(basic_auth)
}

pub mod cors;
