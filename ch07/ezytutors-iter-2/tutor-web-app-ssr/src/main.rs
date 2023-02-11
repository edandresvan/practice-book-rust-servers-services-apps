use std::env;

use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use tera::Tera;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tutor {
  pub name: String,
}

async fn handler_post_tutor(
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
} // end fn handler_post_tutor()

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
  config.service(
    web::scope("")
      .service(web::resource("/").route(web::get().to(index)))
      .service(web::resource("/tutors").route(web::post().to(handler_post_tutor))),
  );
} // end fn app_config()

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let socket_address =
    env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

  println!("Listening on: {socket_address}, open browser and visit");

  HttpServer::new(|| {
    // Create the tera instance
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();

    App::new()
      // inject the tera template engine
      .app_data(web::Data::new(tera))
      .configure(app_config)
  })
  .bind(socket_address)?
  .run()
  .await
} // end fn main()

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::dev::{Service, ServiceResponse};
  use actix_web::http::{header::HeaderValue, header::CONTENT_TYPE, StatusCode};
  use actix_web::test;
  //use actix_web::

  use actix_web::web::{Data, Form};

  #[actix_rt::test]
  async fn test_handle_post_unit_test_1() {
    let params = Form(Tutor {
      name: "Heny".to_string(),
    });

    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();

    let webdata_tera = web::Data::new(tera);

    let response = handler_post_tutor(webdata_tera, params).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
      response.headers().get(CONTENT_TYPE).unwrap(),
      HeaderValue::from_static("text/html")
    );
  } // end fn test_handle_post_test_1()

  #[actix_rt::test]
  async fn test_handle_post_integration_test_1() {
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();

    let app =
      test::init_service(App::new().app_data(Data::new(tera)).configure(app_config))
        .await;

    let request = test::TestRequest::post()
      .uri("/tutors")
      .set_form(&Tutor {
        name: "Terry".to_string(),
      })
      .to_request();

    let response: ServiceResponse = app.call(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
      response.headers().get(CONTENT_TYPE).unwrap(),
      HeaderValue::from_static("text/html")
    );
  }
}
