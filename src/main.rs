#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("web/index.html").await.ok()
}

#[get("/styles.css")]
async fn css() -> Option<NamedFile> {
    NamedFile::open("web/styles.css").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, css])
}
