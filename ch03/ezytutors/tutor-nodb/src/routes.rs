use crate::handlers::*;
use actix_web::web;

pub fn general_routes(config: &mut web::ServiceConfig) {
  config.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(config: &mut web::ServiceConfig) {
  config.service(web::scope("/courses").route("/", web::post().to(new_course)));
}
