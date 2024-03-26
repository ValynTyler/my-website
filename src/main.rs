#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "
    - itch: https://valyn-tyler.itch.io
    - twitter: https://twitter.com/ValynTyler
    - discord: `valyntyler`
    "
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
