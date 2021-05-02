use super::super::diesel::prelude::*;
use dal::{establish_connection, models::{Auth}};
use dal::schema::auths::dsl::*;
use super::super::libs::responders::EZRespond;
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
    let conn = establish_connection();

    let auth = auths
        .filter(email.eq(&login_payload.email))
        .first::<Auth>(&conn);

    if let Ok(auth) = auth {
        if let Ok(true) = bcrypt::verify(login_payload.password.clone(), &auth.password_hash) {
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
