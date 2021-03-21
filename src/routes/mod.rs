use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::templates::Template;
use std::path::{Path, PathBuf};

use crate::db;
use crate::db::types::DbConn;

#[get("/")]
pub fn index(db: State<DbConn>) -> Template {
  let threads = db::get_threads(&db.db);
  Template::render("index", &threads)
}

#[get("/t/<thread>")]
pub fn thread(db: State<DbConn>, thread: String) -> Template {
  let thread = db::get_thread(&db.db, thread);
  Template::render("thread", &thread)
}

#[get("/static/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/files/<file..>")]
pub fn ugc(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("files/").join(file)).ok()
}
