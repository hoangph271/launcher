use std::env::current_dir;
use std::path::PathBuf;

pub fn cwd() -> PathBuf {
    current_dir().unwrap()
}

pub fn bins() -> PathBuf {
    cwd().join(PathBuf::from("bins"))
}

pub fn init_app() {
    use std::fs;

    // ? Create "bins" directory
    fs::create_dir_all(bins()).unwrap();
}
