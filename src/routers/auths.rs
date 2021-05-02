use super::super::diesel::prelude::*;
use super::super::libs;
use super::super::libs::models::Auth;
use super::super::libs::responders::EZRespond;
use super::super::libs::schema::auths::dsl::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::http::Status;
use rocket_contrib::json::*;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
}

#[post("/", data = "<login_payload>")]
pub fn login<'r>(login_payload: Json<LoginPayload>) -> EZRespond<'r> {
    let conn = libs::establish_connection();

    let auth = auths
        .filter(email.eq(&login_payload.email))
        .first::<Auth>(&conn);

    if let Ok(auth) = auth {
        // TODO: Hash this
        if auth.password_hash.eq(&login_payload.password) {
            let claims = Claims { sub: auth.email };

            let jwt_secret = env::var("JWT_SECRET").expect("env::var JWT_SECRET failed...!");
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(&jwt_secret.as_bytes()),
            );

            if let Ok(token) = token {
                return EZRespond::json(json!({ "token": token }), Some(Status::Created));
            }
        }

        EZRespond::by_status(Status::Unauthorized)
    } else {
        EZRespond::by_status(Status::Unauthorized)
    }
}
