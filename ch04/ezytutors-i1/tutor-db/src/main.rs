use chrono::NaiveDateTime;
use dotenvy::dotenv;
use sqlx::postgres::PgPool;
use sqlx::Pool;
use sqlx::Postgres;
use std::env;
use std::io;

/// Represents a course dictated by a tutor.
#[derive(Debug)]
pub struct Course {
  /// Unique identifier (ID) of the tutor.
  pub tutor_id: u32,
  /// Unique identifier (ID) of the course.
  pub course_id: u32,
  /// Name of the course.
  pub course_name: String,
  /// Timestamp when the course was created.
  pub posted_time: Option<NaiveDateTime>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
  // Load environment variables from the .env file
  dotenv().expect(".env file not found");

  // Load the database path
  let database_url: String =
    env::var("DATABASE_URL").expect("DATABASE_URL value is not set in the .env file");

  // Create a database connection pool for the Actix threads
  let db_pool: Pool<Postgres> = PgPool::connect(&database_url).await.unwrap();

  // SQL query to retrieve course rows
  let course_id_sql: i32 = 1;
  let course_rows = sqlx::query!(
    r#"SELECT course_id, tutor_id, course_name, posted_time FROM course_ch04 WHERE course_id = $1"#,
    course_id_sql
  ).fetch_all(&db_pool).await.unwrap();

  // Create a collection of courses object from the courses rows
  let mut courses_list = vec![];
  for course_row in course_rows {
    courses_list.push(Course {
      course_id: course_row.course_id as u32,
      tutor_id: course_row.tutor_id as u32,
      course_name: course_row.course_name,
      posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    })
  }

  // Display the courses
  println!("Courses:\n{:?}", &courses_list);
  Ok(())
}
