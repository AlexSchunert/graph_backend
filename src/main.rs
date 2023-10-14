
#![feature(proc_macro_hygiene, decl_macro)]



#[macro_use]
extern crate rocket;
use std::path::{Path, PathBuf};
use std::io;
use rocket::{response::NamedFile, response::Responder};

/* 
#[get("/world")]
fn world() -> &'static str {
    "Hello, Rocket!"
}
*/

#[get("/")]
fn index() ->  Option<NamedFile> {
    NamedFile::open(Path::new("../../Frontend/graph_frontend/index.html")).ok()
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("../../Frontend/graph_frontend/").join(file)).ok()
}

fn main() {
    
    rocket::ignite().mount("/", routes![index, files]).launch();
    //rocket_instance.mount("../../Frontend/graph_frontend/index.html", File)
}

/*
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
 */
 

 /*
 fn main() {

    println!("Hello World");
 }
*/