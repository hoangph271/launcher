use rocket::request;
use rocket::request::{FromRequest, Request};
use rocket::Outcome;
use std::convert::Infallible;

#[derive(Debug)]
pub enum RangeFromHeader {
    Nope,
    OpenEnd(u64),
    ClosedEnd(u64, u64),
}

impl<'a, 'r> FromRequest<'a, 'r> for RangeFromHeader {
    type Error = Infallible;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let range_header = request.headers().get_one("range");

        if let None = range_header {
            return Outcome::Success(RangeFromHeader::Nope);
        }

        let range = &(range_header.unwrap())["bytes=".len()..];
        let ranges: Vec<Result<u64, core::num::ParseIntError>> = String::from(range)
            .split("-")
            .map(|s| str::parse::<u64>(s))
            .collect();

        if let Ok(start) = ranges[0] {
            if ranges.len() < 2 {
                return Outcome::Success(RangeFromHeader::OpenEnd(start));
            }

            if let Ok(end) = ranges[1] {
                Outcome::Success(RangeFromHeader::ClosedEnd(start, end))
            } else {
                Outcome::Success(RangeFromHeader::OpenEnd(start))
            }
        } else {
            Outcome::Success(RangeFromHeader::Nope)
        }
    }
}
