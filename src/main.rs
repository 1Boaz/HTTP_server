use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn return_html() -> impl Responder {
    HttpResponse::Ok().body(r#"
    <!doctype html>
    <html>
    <head>
    <meta charset="utf-8">
    </head>
    <body>
    <a href="https://www.rust-lang.org">rust-lang</a>"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(return_html)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}