use std::{fs::File, io::Read};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "
    
    "
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
