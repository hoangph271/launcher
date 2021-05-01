use super::super::libs::responders::EZRespond;
use rocket::{http::Status, request::*, Request, Response};
use serde::*;
use std::env;
use std::io::Cursor;
use std::str;
use sys_info;

#[catch(404)]
pub fn not_found<'r>() -> EZRespond<'r> {
    EZRespond::by_status(Status::ImATeapot)
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

#[derive(Debug, Serialize)]
struct DiskInfo {
    total: u64,
    free: u64,
}
#[derive(Debug, Serialize)]
struct LoadAvg {
    fifteen: f64,
}
#[derive(Debug, Serialize)]
struct MemInfo {
    total: u64,
    avail: u64,
    free: u64,
}
#[derive(Debug, Serialize)]
struct OsInfo {
    release: String,
    #[serde(rename = "type")]
    os_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
    username: String,
    #[serde(rename = "diskUsage")]
    disk_info: DiskInfo,
    load_avg: LoadAvg,
    memory: MemInfo,
    hostname: String,
    #[serde(rename = "cpuCount")]
    cpu_num: u32,
    os: OsInfo,
}

fn get_system_status(basic_auth: &BasicAuth) -> Result<SystemStatus, ()> {
    let err_mapper = |e| {
        dbg!(e);
        ()
    };

    let disk_info = sys_info::disk_info().map_err(err_mapper)?;
    let load_avg = sys_info::loadavg().map_err(err_mapper)?;
    let memory = sys_info::mem_info().map_err(err_mapper)?;
    let hostname = sys_info::hostname().map_err(err_mapper)?;
    let cpu_num = sys_info::cpu_num().map_err(err_mapper)?;
    let os_release = sys_info::os_release().map_err(err_mapper)?;
    let os_type = sys_info::os_type().map_err(err_mapper)?;

    Ok(SystemStatus {
        username: basic_auth.username.clone(),
        disk_info: DiskInfo {
            total: disk_info.total,
            free: disk_info.free,
        },
        load_avg: LoadAvg {
            fifteen: load_avg.fifteen,
        },
        memory: MemInfo {
            total: memory.total,
            avail: memory.avail,
            free: memory.free,
        },
        hostname: hostname,
        cpu_num: cpu_num,
        os: OsInfo {
            release: os_release,
            os_type: os_type,
        },
    })
}

#[get("/")]
pub fn status<'r>(basic_auth: BasicAuth) -> EZRespond<'r> {
    match get_system_status(&basic_auth) {
        Ok(system_status) => {
            EZRespond::json(json!(system_status), None)
        }
        Err(_) => EZRespond::by_status(Status::InternalServerError),
    }
}
