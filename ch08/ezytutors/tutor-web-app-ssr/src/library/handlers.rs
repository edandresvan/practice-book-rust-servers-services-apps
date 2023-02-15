use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Result;

use crate::errors::EzyTutorError;

/// Handles the request to show the registration form.
pub async fn show_register_form(
  template_engine: web::Data<tera::Tera>
) -> Result<HttpResponse, actix_web::error::Error> {
  let mut context = tera::Context::new();

  context.insert("errors", "");

  context.insert("current_username", "");
  context.insert("current_password", "");
  context.insert("current_password", "");
  context.insert("current_confirmation", "");
  context.insert("current_name", "");
  context.insert("current_image_url", "");
  context.insert("current_profile", "");

  let contents: String = template_engine
    .render("register.html", &context)
    .map_err(|error| EzyTutorError::TeraError(format!("Tera template error: {error}")))?;

  Ok(HttpResponse::Ok().content_type("text/html").body(contents))
} // end fn show_register_form()

/// Handles the requests to process a registration.
pub async fn handle_register() -> Result<HttpResponse, actix_web::error::Error> {
  Ok(HttpResponse::Ok().body(""))
} // end fn handle_register()
