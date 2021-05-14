use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub image: Option<String>,
}

#[derive(Debug, Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct UserData<'a> {
    pub id: &'a str,
    pub email: &'a str,
    pub name: &'a str,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "auths"]
pub struct AuthData<'a> {
    pub id: &'a str,
    pub auth_type: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Auth {
    pub id: String,
    pub auth_type: String,
    pub email: String,
    pub password_hash: String,
}
