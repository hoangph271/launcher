use rocket::{http::Status, request::*, Request, Response};
use rocket_contrib::json::*;
use std::env;
use std::io::Cursor;
use std::str;

#[catch(404)]
pub fn not_found<'r>() -> Response<'r> {
    Response::build()
        .status(Status::ImATeapot)
        .sized_body(Cursor::new("418 | I'm a teapot"))
        .finalize()
}

#[catch(401)]
pub fn unauthorized<'r>() -> Response<'r> {
    Response::build()
        .status(Status::Unauthorized)
        .sized_body(Cursor::new("401 | Unauthorized"))
        .finalize()
}

#[derive(Debug)]
pub struct BasicAuth {
    username: String,
    password: String,
}
impl<'a, 'r> BasicAuth {
    fn from_request_wrapped(request: &'a Request<'r>) -> Result<Outcome<Self, ()>, Status> {
        let authorization = request
            .headers()
            .get_one("Authorization")
            .ok_or(Status::Unauthorized)?;

        let credential_bytes = &authorization["Basic ".len()..];
        let header_bytes = base64::decode(credential_bytes).or(Err(Status::Unauthorized))?;
        let header_str = str::from_utf8(&header_bytes).or(Err(Status::Unauthorized))?;
        let header_fields = str::split(header_str, ':')
            .map(|str| str.to_owned())
            .collect::<Vec<String>>();

        let outcome = Outcome::Success(BasicAuth {
            username: header_fields.get(0).ok_or(Status::Unauthorized)?.to_owned(),
            password: header_fields.get(1).ok_or(Status::Unauthorized)?.to_owned(),
        });

        Ok(outcome)
    }
}
impl<'a, 'r> FromRequest<'a, 'r> for BasicAuth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        match BasicAuth::from_request_wrapped(&request) {
            Ok(outcome) => {
                if let (Ok(username), Ok(password)) = (
                    env::var("STATUS_CHECK_USERNAME"),
                    env::var("STATUS_CHECK_PASSWORD"),
                ) {
                    if let Outcome::Success(basic_auth) = outcome {
                        let credentials_match =
                            basic_auth.username.eq(&username) && basic_auth.password.eq(&password);

                        if credentials_match {
                            return Outcome::Success(basic_auth);
                        }
                    }
                }

                Outcome::Failure((Status::Unauthorized, ()))
            }
            Err(status) => Outcome::Failure((status, ())),
        }
    }
}

#[get("/")]
pub fn status<'r>(basic_auth: BasicAuth) -> Result<JsonValue, Status> {
    use sys_info;

    let err_mapper = |e| {
        dbg!(e);
        Status::BadRequest
    };

    let disk_info = sys_info::disk_info().map_err(err_mapper)?;
    let load_avg = sys_info::loadavg().map_err(err_mapper)?;
    let memory = sys_info::mem_info().map_err(err_mapper)?;

    let json = json!({
        "username": basic_auth.username,
        "hostname": sys_info::hostname().map_err(err_mapper)?,
        "cpuCount": sys_info::cpu_num().map_err(err_mapper)?,
        "diskUsage": {
            "total": disk_info.total,
            "free": disk_info.free
        },
        "loadAvg": load_avg.fifteen,
        "memory": {
            "total": memory.total,
            "free": memory.free,
            "available": memory.avail
        },
        "os": {
            "release": sys_info::os_release().map_err(err_mapper)?,
            "type": sys_info::os_type().map_err(err_mapper)?
        }
    });

    Ok(json)
}
