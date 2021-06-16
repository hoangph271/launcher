use crate::libs::responders::EZRespond;
use rocket::http::Status;

#[catch(401)]
pub fn unauthorized<'r>() -> EZRespond<'r> {
    EZRespond::by_status(Status::Unauthorized)
}
