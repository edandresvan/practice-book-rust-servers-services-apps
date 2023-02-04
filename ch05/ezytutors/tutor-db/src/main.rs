use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use dotenvy::dotenv;
use sqlx::postgres::PgPool;
use sqlx::Pool;
use sqlx::Postgres;
use std::env;
use std::io;
use std::sync::Mutex;

mod db_access;
mod handlers;
mod models;
mod routes;
mod state;
mod errors;
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
      .configure(general_routes)
      .configure(course_routes)
  };

  // Start the server
  let server_socket = "127.0.0.1:3000";
  HttpServer::new(app).bind(&server_socket)?.run().await
}
