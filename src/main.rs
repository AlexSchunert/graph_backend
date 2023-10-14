
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;




#[get("/world")]
fn world() -> &'static str {
    "Hello, Rocket!"
}


fn main() {
    rocket::ignite().mount("/", routes![world]).launch();
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