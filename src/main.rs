use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use std::fs;

const ARGS: Vec<String> = std::env::args().collect::<Vec<String>>();

#[get("/")]
async fn return_html() -> impl Responder {
    let body = fs::read_to_string(&ARGS[0]);
    match body {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(_) => HttpResponse::Ok().content_type("text/html").body("was not able to read file"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(return_html)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}