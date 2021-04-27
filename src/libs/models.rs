use super::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub nickname: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct UserData<'a> {
    pub id: &'a str,
    pub email: &'a str,
    pub nickname: &'a str,
}
