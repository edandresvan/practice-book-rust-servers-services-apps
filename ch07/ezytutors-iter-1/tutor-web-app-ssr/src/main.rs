use std::env;

use actix_files as fs;
use actix_web::{error, web, App, Error, HttpResponse, HttpServer, Result};
use tera::Tera;

async fn index(template_engine: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
  // A context to inject data into a web page
  let mut context = tera::Context::new();

  // Assign a value to the 'name' variable which is use in the HTML template file
  context.insert("name", "Bobby Lond");

  let body_contents: String = template_engine
    .render("index.html", &context)
    .map_err(|_| error::ErrorInternalServerError("Error in the tera template."))?;

  Ok(
    HttpResponse::Ok()
      .content_type("text/html")
      .body(body_contents),
  )
} // end fn index()

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let socket_address =
    env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

  println!("Listening on: {socket_address}, open browser and visit");

  HttpServer::new(|| {
    // Create the tera instance
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();

    App::new()
      .app_data(web::Data::new(tera))
      .service(fs::Files::new("/static", "./static").show_files_listing())
      .service(web::resource("/").route(web::get().to(index)))
  })
  .bind(socket_address)?
  .run()
  .await
}
