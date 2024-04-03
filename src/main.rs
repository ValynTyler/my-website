#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("index.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
