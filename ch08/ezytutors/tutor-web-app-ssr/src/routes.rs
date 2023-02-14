use crate::handlers::{handle_register, show_register_form};
use actix_web::web;

/// Configures the routes for the web application.
///
/// # Arguments
///
/// * `config`: Configuration of the application.
pub fn app_config(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("")
      // Route for static resources.
      .service(actix_files::Files::new("/static", "./static").show_files_listing())
      // Route for showing the registration form.
      .service(web::resource("/").route(web::get().to(show_register_form)))
      // Route for processing the registration action.
      .service(web::resource("/register").route(web::post().to(handle_register))),
  );
} // end fn app_config()
