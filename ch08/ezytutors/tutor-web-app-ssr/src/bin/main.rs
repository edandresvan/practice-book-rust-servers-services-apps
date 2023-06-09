use dotenvy::dotenv;
use std::env;

use actix_web::{
  web::{self, Data},
  App, HttpServer,
};

use sqlx::PgPool;
use tera::Tera;

use tutor_ssr::routes::app_config;
use tutor_ssr::state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Load environment variables from the .env file
  dotenv().expect(".env file not found");
  // Load the database path
  let database_url: String =
    env::var("DATABASE_URL").expect("'DATABASE_URL' is missing in the .env file");
  // Load the socket address
  let socket_address =
    env::var("SERVER_ADDRESS").expect("'SERVER_ADDRESS' is missing in the .env file");

  println!("Tutor Web App SSR is listening on: {socket_address}, open browser and visit");

  // Create the application state
  let pg_pool = PgPool::connect(&database_url).await.unwrap();
  let shared_data = web::Data::new(AppState { db_pool: pg_pool });

  // Configure the web application with the template engine and the application configuration.
  HttpServer::new(move || {
    let tera_engine =
      Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*.html")).unwrap();

    App::new()
      .app_data(Data::new(tera_engine))
      .app_data(shared_data.clone())
      .configure(app_config)
  })
  .bind(&socket_address)?
  .run()
  .await
} // end fn main()
