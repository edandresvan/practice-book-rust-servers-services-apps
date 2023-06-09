use actix_web::web::Data;
use actix_web::{self, web, App, HttpServer};
use std::sync::Mutex;
use std::{io, vec};

#[path = "../handlers.rs"]
mod handlers;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

#[path = "../models.rs"]
mod models;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
  // Initialize the state object for the application
  let shared_data: Data<AppState> = web::Data::new(AppState {
    health_check_response: "I'm good. You've already asked me ".to_string(),
    visit_count: Mutex::new(0),
    courses: Mutex::new(vec![]),
  });

  // Create the web application with state and routes
  let app = move || {
    App::new()
      .app_data(shared_data.clone())
      .configure(general_routes)
      .configure(course_routes)
  };

  // Create and start the web server with the web application
  HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
