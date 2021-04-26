use super::super::app_context::bins;
use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::path::PathBuf;
use std::fs::{File, Metadata, read_dir};
use std::time::UNIX_EPOCH;

#[derive(Debug, Serialize)]
pub struct FSEntry {
    key: String,
    is_dir: bool,
    size: Option<u64>,
    mime: Option<String>,
    created: u128,
    modified: u128,
    children: Option<Vec<FSEntry>>,
}
fn read_entry(path: PathBuf) -> FSEntry {
    let file_path = bins().join(path.to_owned());
    let file = File::open(file_path).unwrap();
    let metadata = file.metadata().unwrap();

    let mime = mime_guess::from_path(path.to_owned()).first();
    let (created, modified) = extract_timestamps(&metadata);

    FSEntry {
        key: path.to_string_lossy().as_ref().to_string(),
        is_dir: metadata.is_dir(),
        children: None,
        mime: if let Some(mime) = mime {
            Some(mime.as_ref().to_string())
        } else {
            None
        },
        size: Some(metadata.len()),
        created,
        modified,
    }
}
fn extract_timestamps(metadata: &Metadata) -> (u128, u128) {
    let created = metadata
        .created()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let modified = metadata
        .modified()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    (created, modified)
}

pub struct DirsResponder {
    path: PathBuf,
}
impl DirsResponder {
    pub fn new(path: PathBuf) -> Self {
        DirsResponder { path }
    }
}
impl<'a> Responder<'a> for DirsResponder {
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        dirs(self.path).respond_to(req)
    }
}

pub fn dirs(path: PathBuf) -> Json<FSEntry> {
    let item_path = bins().join(path.to_owned());
    let mut fs_entry = read_entry(path);

    let children = read_dir(item_path)
        .unwrap()
        .map(|entry| {
            read_entry(
                entry
                    .unwrap()
                    .path()
                    .strip_prefix(bins())
                    .unwrap()
                    .to_path_buf(),
            )
        })
        .collect::<Vec<FSEntry>>();

    fs_entry.children = Some(children);

    Json(fs_entry)
}
