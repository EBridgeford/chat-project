#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::NamedFile;

use std::io;
use std::path::{PathBuf, Path};
use rocket_contrib::helmet::SpaceHelmet;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("web/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("web/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files])
        .attach(SpaceHelmet::default())
        //.mount("/web", StaticFiles::from("/web"))
        .launch();
}
