use crate::handlers::*;
use actix_web::web;

pub fn general_routes(config: &mut web::ServiceConfig) {
  config.route("/health", web::get().to(health_check_handler));
}
