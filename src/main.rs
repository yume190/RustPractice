use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

// this handler gets called if the query deserializes into `Info` successfully
// otherwise a 400 Bad Request error response is returned
#[get("/")]
async fn index(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
