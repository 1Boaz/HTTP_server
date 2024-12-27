use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use std::fs;
use std::env;

// Func to read HTML file and send it to the browser
#[get("/")]
async fn return_html() -> impl Responder {
    let mut file_path = "index.html";
    // Read the file path from command-line arguments.
    let args: Vec<String> = env::args().collect();
    // checks if was an input
    if args.len() > 2 {
        file_path = &args[1];
    }
    // logs the HTML
    println!("read file: {}", &file_path);
    // returns the HTML
    match fs::read_to_string(file_path) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(_) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body("Could not read the specified file."),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // creates the default host and port
    let mut address = "127.0.0.1";
    let mut port: u16 = 8080;
    // takes args and puts them in place(in case they were inserted)
    let args: Vec<String> = env::args().collect();
    if args.len() > 3 {
        if args[2] != "" {
            address = args[2].as_str();
        }
        if args[3] != "" {
            port = (&args[3]).parse().unwrap();
        }
    }
    // starts the server and creates the return_html route
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(return_html)
    })
        .bind((address, port))?
        .run()
        .await
}