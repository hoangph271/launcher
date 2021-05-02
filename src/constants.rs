pub mod response_messsage {
    pub const OK: &str = "200 | OK";
    pub const CREATED: &str = "201 | Created";
    pub const UNAUTHORIZED: &str = "401 | Unauthorized";
    pub const NOT_FOUND: &str = "404 | Not Found";
    pub const CONFLICT: &str = "409 | Conflict";
    pub const IM_A_TEAPOT: &str = "418 | I'm a teapot";
    pub const INTERNAL_SERVER_ERROR: &str = "500 | Internal Server Error";
}

pub mod auth_type {
    pub const BASIC: &str = "basic";
}
