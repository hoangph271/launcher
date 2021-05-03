use super::super::libs::responders::EZRespond;
use rocket::http::Status;
use rocket_contrib::json::*;

pub fn with_json_catcher<'a, T>(
    json: Result<Json<T>, JsonError>,
    executor: impl FnOnce(Json<T>) -> EZRespond<'a>,
) -> EZRespond<'a> {
    match json {
        Ok(json) => executor(json),
        Err(e) => {
            if let JsonError::Parse(_, e) = e {
                EZRespond::text(format!("{}", e), Some(Status::UnprocessableEntity))
            } else {
                EZRespond::by_status(Status::UnprocessableEntity)
            }
        }
    }
}
