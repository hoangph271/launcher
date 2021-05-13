#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod app_context;
mod constants;
mod guards;
mod libs;
mod routers;

use app_context::{bins, init_app};
use rocket_contrib::serve::StaticFiles;
use routers::{auths, dirs, others, streams, users};

fn launch_app() -> rocket::Rocket {
    rocket::ignite()
        .mount("/bins", StaticFiles::from(bins()))
        .mount(
            "/users",
            routes![
                users::get_user,
                users::get_users,
                users::post_user,
                users::delete_user,
                users::update_user,
                users::update_user_image,
            ],
        )
        .mount("/auths", routes![auths::login])
        .mount(
            "/streams",
            routes![streams::stream_down, streams::stream_up],
        )
        .mount("/dirs", routes![dirs::get_dir, dirs::get_index_dir])
        .mount("/status", routes![others::server_status])
        .register(catchers![
            others::not_found,
            others::unauthorized,
            others::unprocessable_entity
        ])
        .attach(others::cors::Cors)
}

fn main() {
    init_app();
    launch_app().launch();
}

#[cfg(test)]
#[macro_use]
extern crate diesel_migrations;

#[cfg(test)]
mod main_tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn it_works() {}

    #[test]
    fn can_use_memory_db() {
        embed_migrations!("./dal/migrations");

        use dal::establish_connection;
        use diesel::prelude::*;
        use std::env;

        env::set_var("DATABASE_URL", ":memory:");
        let conn = establish_connection();
        conn.execute(
            "SELECT 
            name
        FROM 
            sqlite_master 
        WHERE 
            type ='table' AND 
            name NOT LIKE 'sqlite_%';",
        )
        .unwrap();
    }

    #[test]
    fn get_non_exists_route() {
        let app = launch_app();
        let client = Client::new(app).unwrap();

        let mut res = client.get("/418").dispatch();

        assert_eq!(res.status(), Status::ImATeapot);
        assert_eq!(res.body_string(), Some(String::from("418 | I'm a teapot")));
    }
}
