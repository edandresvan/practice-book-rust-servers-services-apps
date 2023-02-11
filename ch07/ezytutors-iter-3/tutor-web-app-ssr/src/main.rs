use std::env;

use actix_web::{
  web::{self, Data},
  App, HttpResponse, HttpServer, Result,
};
use serde::{Deserialize, Serialize};
use tera::Tera;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tutor {
  pub name: String,
}

async fn handle_get_tutors(
  template_engine: web::Data<tera::Tera>
) -> Result<HttpResponse, actix_web::error::Error> {
  let tutors: Vec<Tutor> = vec![Tutor {
    name: "Tutor 1".to_string(),
  }];

  let mut context = tera::Context::new();
  context.insert("tutors", &tutors);

  let html_contents: String = template_engine
    .render("list.html", &context)
    .map_err(|_| actix_web::error::ErrorInternalServerError("tera template error."))?;

  Ok(
    HttpResponse::Ok()
      .content_type("text/html")
      .body(html_contents),
  )
} // end fn handle_get_tutors()

async fn handle_post_tutor(
  template_engine: web::Data<tera::Tera>,
  params: web::Form<Tutor>,
) -> Result<HttpResponse, actix_web::error::Error> {
  let mut context = tera::Context::new();
  context.insert("name", &params.name);
  context.insert("text", "Welcome!!!");

  let body_contents: String = template_engine
    .render("user.html", &context)
    .map_err(|_| actix_web::error::ErrorInternalServerError("tera template error"))?;

  Ok(
    HttpResponse::Ok()
      .content_type("text/html")
      .body(body_contents),
  )
} // end fn handle_post_tutor()

async fn index(
  template_engine: web::Data<tera::Tera>
) -> Result<HttpResponse, actix_web::error::Error> {
  let body_contents: String = template_engine
    .render("form.html", &tera::Context::new())
    .map_err(|_| {
      actix_web::error::ErrorInternalServerError("Error in the tera template.")
    })?;

  Ok(
    HttpResponse::Ok()
      .content_type("text/html")
      .body(body_contents),
  )
} // end fn index()

fn app_config(config: &mut web::ServiceConfig) {
  config.service(web::scope("").service(web::resource("/").route(web::get().to(index))));

  config.service(
    web::scope("/tutors")
      .route("/", web::get().to(handle_get_tutors))
      .route("/", web::post().to(handle_post_tutor)),
  );
} // end fn app_config()

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let socket_address =
    env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

  println!("Listening on: {socket_address}, open browser and visit");

  // HttpServer::new(|| {
  //   // Create the tera instance
  //   let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();

  //   App::new()
  //     // inject the tera template engine
  //     .app_data(web::Data::new(tera))
  //     .configure(app_config)
  // })
  // .bind(socket_address)?
  // .run()
  // .await

  use actix_files as fs;
  HttpServer::new(|| {
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();

    App::new()
      .app_data(Data::new(tera))
      .service(fs::Files::new("/static", "./static").show_files_listing())
      .service(web::resource("/tutors").route(web::get().to(handle_get_tutors)))
  })
  .bind(socket_address)?
  .run()
  .await
} // end fn main()
