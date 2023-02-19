use crate::handlers::auth::{
  handle_register, handle_signin, show_register_form, show_signin_form,
};

use crate::handlers::course::{
  handle_delete_course, handle_insert_course, handle_update_course,
};

use actix_web::web;

/// Configures the basic routes of the web application.
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

/// Configures the routes of the web application for course management.
///
/// # Arguments
///
/// * `config`: Configuration of the application.
pub fn course_config(config: &mut web::ServiceConfig) {
  config.service(
    // Define '/courses' as the base path for course management.
    web::scope("/courses")
      // Route for creating a new course
      .service(
        web::resource("/new/{tutor_id}").route(web::post().to(handle_insert_course)),
      )
      // Route for updating an existing course.
      .service(
        web::resource("{tutor_id}/{course_id}")
          .route(web::put().to(handle_update_course)),
      )
      // Route for deleting an existing course.
      .service(
        web::resource("delete/{tutor_id}/{course_id}")
          .route(web::delete().to(handle_delete_course)),
      ),
  );
} // end fn course_config()
