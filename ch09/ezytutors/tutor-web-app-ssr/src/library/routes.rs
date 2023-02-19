use crate::handlers::auth::{
  handle_register, handle_signin, show_register_form, show_signin_form,
};
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
      .service(web::resource("/register").route(web::post().to(handle_register)))
      // Route for showing the sign-in form
      .service(web::resource("/signinform").route(web::get().to(show_signin_form)))
      // Route for processing the sign-in action
      .service(web::resource("/signin").route(web::post().to(handle_signin))),
  );
} // end fn app_config()
