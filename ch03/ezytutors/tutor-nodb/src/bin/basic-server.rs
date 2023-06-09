use std::io;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};

/// Configure the routes.
pub fn general_routes(config: &mut web::ServiceConfig) {
  config.route("/health", web::get().to(health_check_handler));
}

/// Configure the health handler
pub async fn health_check_handler() -> impl Responder {
  HttpResponse::Ok().json("Hello. EzyTutors is alive and kicking")
}

/// Main function to start the web server.
#[actix_rt::main]
async fn main() -> io::Result<()> {
  // Create the app with its routes
  let app = move || App::new().configure(general_routes);

  // Start the server
  let server_address: &str = "127.0.0.1:3000";
  HttpServer::new(app).bind(&server_address)?.run().await
}
