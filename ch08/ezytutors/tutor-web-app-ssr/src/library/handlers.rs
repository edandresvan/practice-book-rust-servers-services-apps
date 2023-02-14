use actix_web::HttpResponse;
use actix_web::Result;

/// Handles the request to show the registration form.
pub async fn show_register_form() -> Result<HttpResponse, actix_web::error::Error> {
  let contents: &str = "Hello, this is the registration page";
  Ok(HttpResponse::Ok().content_type("text/html").body(contents))
} // end fn show_register_form()

/// Handles the requests to process a registration.
pub async fn handle_register() -> Result<HttpResponse, actix_web::error::Error> {
  Ok(HttpResponse::Ok().body(""))
} // end fn handle_register()
