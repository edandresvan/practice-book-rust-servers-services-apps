use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use dotenvy::dotenv;
use errors::EzyTutorError;
use sqlx::postgres::PgPool;
use sqlx::Pool;
use sqlx::Postgres;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../db_access/mod.rs"]
mod db_access;
#[path = "../errors.rs"]
mod errors;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;
use crate::routes::*;
use crate::state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
  // Load environment variables from the .env file
  dotenv().expect(".env file not found");

  // Load the database path
  let database_url: String =
    env::var("DATABASE_URL").expect("DATABASE_URL value is not set in the .env file");

  // Create a database connection pool for the Actix threads
  let db_pool: Pool<Postgres> = PgPool::connect(&database_url).await.unwrap();

  // Create the application state
  let shared_data = web::Data::new(AppState {
    health_check_response: "I'm good. You've already asked me ".to_string(),
    visit_count: Mutex::new(0),
    db: db_pool,
  });

  // Create the application
  let app = move || {
    App::new()
      .app_data(shared_data.clone())
      .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
        EzyTutorError::InvalidInput("Please provide valid Json input".to_string()).into()
    }))
      .configure(general_routes)
      .configure(course_routes)
      .configure(tutor_routes)
  };

  // Start the server
  let server_socket = "127.0.0.1:3000";
  HttpServer::new(app).bind(&server_socket)?.run().await
}
