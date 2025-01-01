use actix_files::NamedFile;
use actix_web::middleware::Logger;
use actix_web::HttpRequest;
use actix_web::{web, App, HttpServer};
use std::env;
use std::path::PathBuf;

async fn index(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse()?;
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // creates the default host and port
    let mut address = "127.0.0.1";
    let mut port: u16 = 8080;
    let mut file_path = "index.html";
    // takes args and puts them in place(in case they were inserted)
    let args: Vec<String> = env::args().collect();
    if args.len() > 3 {
        file_path = &args[1];
        if args[2] != "" {
            address = args[2].as_str();
        }
        if args[3] != "" {
            port = (&args[3]).parse().unwrap();
        }
    }

    HttpServer::new(|| {
        App::new()
            .route(file_path, web::get().to(index))
            .wrap(Logger::default())
    })
    .bind((address, port))?
    .run()
    .await
}
