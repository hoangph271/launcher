use std::env::current_dir;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub fn cwd() -> PathBuf {
    current_dir().expect("failed getting cwd")
}

pub fn bins() -> PathBuf {
    cwd().join(PathBuf::from("bins"))
}

pub fn init_app() {
    dotenv::dotenv().expect("Error loading .env");

    // ? Create "bins" directory
    create_dir_all(bins()).expect("failed creating bins/");
    create_dir_all(bins().join("users")).expect("failed creating bins/users");
}
