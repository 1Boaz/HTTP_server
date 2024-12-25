use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use std::fs;
use std::env;

#[get("/")]
async fn return_html() -> impl Responder {
    // Read the file path from command-line arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Usage: Provide the path to the HTML file as a command-line argument.");
    }

    let file_path = &args[1];
    match fs::read_to_string(file_path) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(_) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body("Could not read the specified file."),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // Initialize logging.

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(return_html)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
