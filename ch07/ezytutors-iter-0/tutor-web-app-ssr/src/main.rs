use std::env;

use actix_files as fs;
use actix_web::{error, web, App, Error, HttpResponse, HttpServer, Result};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let socket_address =
    env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

  println!("Listening on: {socket_address}, open browser and visit");

  HttpServer::new(|| {
    App::new().service(fs::Files::new("/static", "./static").show_files_listing())
  })
  .bind(socket_address)?
  .run()
  .await
}
