use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
#[derive(Deserialize)]
struct Info {
    a: u64,
    b: u64,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK!")
}

#[post("/add")]
async fn add(req_body: web::Json<Info>) -> impl Responder {
    let sum = req_body.a + req_body.b;
    HttpResponse::Ok().json(sum)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health).service(add))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
