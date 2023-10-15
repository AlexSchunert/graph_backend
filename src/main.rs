#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
//use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::content::Json;
use rocket::response::NamedFile;
//use std::io;
use std::path::{Path, PathBuf};

/*
#[get("/world")]
fn world() -> &'static str {
    "Hello, Rocket!"
}
*/

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("graph_frontend/index.html")).ok()
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("graph_frontend/").join(file)).ok()
}

#[post("/text_file", data = "<input>")]
fn handle_text_file_data(input: String) -> Json<&'static str> {
    let is_valid: bool = false;
    // Check if input is valid graphML here
    // ...
    // Output for debugging
    println!("{}", input);
    let json_data: &str;
    if is_valid == true {
        //rocket::Response::new().status(Status::ok).sized_body()
        json_data = r#"{"status": "success", "message": "Graph successfully processed" }"#;
    } else {
        json_data = r#"{"status": "error", "message": "Encountered a problem" }"#;
    }
    println!("{}", json_data);
    return Json(json_data);
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files, handle_text_file_data])
        .launch();
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
