use actix_files::NamedFile;
use actix_web::{get, Responder};

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("static/index.html").await
}
#[get("/home")]
async fn home() -> impl Responder {
    NamedFile::open_async("static/home.html").await
}