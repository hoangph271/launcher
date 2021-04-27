use serde::{Serialize, Deserialize};
use super::schema::users;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub nickname: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub id: String,
    pub email: &'a str,
    pub nickname: &'a str,
}
