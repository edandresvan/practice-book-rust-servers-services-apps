use crate::models::course::*;
use sqlx::postgres::{PgPool, PgQueryResult};

use crate::errors::EzyTutorError;

/// Gets all courses for the given tutor.
///
/// # Arguments
///
/// * `db_pool`: Database connection pool.
/// * `tutor_id`: Unique identifier (ID) of the tutor.
pub async fn get_courses_for_tutor_db(
  db_pool: &PgPool,
  tutor_id: u32,
) -> Result<Vec<Course>, EzyTutorError> {
  // SQL query to retrieve a collection of courses

  let courses: Vec<Course> = sqlx::query_as!(
    Course, 
    r#"SELECT course_id, tutor_id, course_name as name, posted_time, course_description as "description?", course_format as "format?", course_structure as "structure?", course_duration as "duration?", course_language as "language?", course_level as "level?", course_price as "price?" 
    FROM course_ch06 
    WHERE tutor_id = $1"#, tutor_id as i32
  )  
  .fetch_all(db_pool)
  .await?;

  Ok(courses)
} // end fn get_courses_for_tutor_db()

/// Gets the course for the given tutor and course identifiers.
///
/// # Arguments
///
/// * `db_pool`: Database connection pool.
/// * `tutor_id`: Unique identifier (ID) of the tutor.
/// * `course_id`: Unique identifier (ID) of the course.
pub async fn get_course_details_db(
  db_pool: &PgPool,
  tutor_id: u32,
  course_id: u32,
) -> Result<Vec<Course>, EzyTutorError> {
  // SQL query to retrieve course rows
  let courses: Vec<Course> = sqlx::query_as!(Course, 
    r#"SELECT course_id, tutor_id, course_name as name, posted_time, course_description as "description?", course_format as "format?", course_structure as "structure?", course_duration as "duration?", course_language as "language?", course_level as "level?", course_price as "price?" 
    FROM course_ch06 
    WHERE tutor_id = $1 AND course_id = $2"#, 
    tutor_id as i32, course_id as i32)
    .fetch_all(db_pool)
    .await?;

  if !courses.is_empty() {
    Ok(courses)
  } else {
    Err(EzyTutorError::NotFound(format!(
      "Course id `{course_id}` not found for tutor id `{tutor_id}`.")))
  }
} // end fn get_course_details_db()

/// Creates (inserts) the given course into the database.
///
/// # Arguments
///
/// * `db_pool`: Database connection pool.
/// * `course`: Course to be created.
pub async fn post_new_course_db(
  db_pool: &PgPool,
  course: CreateCourse,
) -> Result<Vec<Course>, EzyTutorError> {
  // Prepare the SQL insert statement
  let courses: Vec<Course> = sqlx::query_as!(Course, 
    r#"INSERT INTO course_ch06 (tutor_id, course_name, course_description, course_format, course_structure, course_duration, course_language, course_level, course_price) 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
    RETURNING course_id, tutor_id, course_name as name, posted_time, course_description as "description?", course_format as "format?", course_structure as "structure?", course_duration as "duration?", course_language as "language?", course_level as "level?", course_price as "price?""#, 
    course.tutor_id, course.name, course.description, course.format, course.structure, course.duration, course.language, course.level, course.price)
    .fetch_all(db_pool)
    .await?;

  Ok(courses)
} // end fn post_new_course_db()

/// Updates the given course parameteres in the database.
///
/// # Arguments
///
/// * `db_pool`: Database connection pool.
/// * `tutor_id`: Tutor ID.
/// * `course_id`: Course ID.
/// * `course`: Course to be updated.
pub async fn update_course_details_db(db_pool: &PgPool, tutor_id: u32, course_id: u32, course: UpdateCourse) -> Result<Vec<Course>, EzyTutorError> {
  // Prepare the SQL select statement to find the existing course

  let mut existing_course = sqlx::query_as!(Course, 
    r#"SELECT course_id, tutor_id, course_name as name, posted_time, course_description as "description?", course_format as "format?", course_structure as "structure?", course_duration as "duration?", course_language as "language?", course_level as "level?", course_price as "price?" 
    FROM course_ch06 
    WHERE tutor_id = $1 AND course_id = $2"#,
  tutor_id as i32, course_id as i32)
  .fetch_one(db_pool)
  .await
  .map_err(|_error| EzyTutorError::NotFound(format!("Course id `{course_id}` not found.")))?;
  
  // Change the existing course fields if there is data
  if course.name.is_some() {
    existing_course.name = course.name.unwrap();
  }

  if course.description.is_some() {
    existing_course.description = course.description;
  }

  if course.format.is_some() {
    existing_course.format = course.format;
  }

  if course.structure.is_some() {
    existing_course.structure = course.structure;
  }

  if course.duration.is_some() {
    existing_course.duration = course.duration;
  }

  if course.language.is_some()  {
      existing_course.language = course.language;
  }

  if course.level.is_some() {
    existing_course.level = course.level;
  }

  if course.price.is_some() {
    existing_course.price = course.price;
  }

  // Prepare the SQL update statement
  let updated_course_result = sqlx::query_as!(Course,
  r#"UPDATE course_ch06 SET 
  course_name = $1, course_description = $2, course_format = $3, 
  course_structure = $4, course_duration = $5, course_language = $6,
  course_level = $7, course_price = $8 
  WHERE tutor_id = $9 AND course_id = $10 
  RETURNING course_id, tutor_id, course_name as name, posted_time, course_description as "description?", course_format as "format?", course_structure as "structure?", course_duration as "duration?", course_language as "language?", course_level as "level?", course_price as "price?" "#,
  existing_course.name, existing_course.description, existing_course.format, 
  existing_course.structure, existing_course.duration, existing_course.language, 
  existing_course.level, existing_course.price, 
  existing_course.tutor_id, existing_course.course_id)
  .fetch_one(db_pool)
  .await;
  
  match updated_course_result {
    Ok(up_course) => Ok(vec![up_course]),
    Err(_error) => Err(EzyTutorError::NotFound(format!("Course id `{course_id}` not found")))
  }
  
} // end fn update_course_details_db()


/// Removes (deletes) the given course parameteres from the database.
///
/// # Arguments
///
/// * `db_pool`: Database connection pool.
/// * `tutor_id`: Tutor ID.
/// * `course_id`: Course ID.
pub async fn delete_course_db(db_pool: &PgPool, tutor_id: u32, course_id: u32) -> Result<String, EzyTutorError> {
  // Prepare the SQL delete statement
  let query_result: PgQueryResult = sqlx::query!(
    r#"DELETE FROM course_ch06 
    WHERE tutor_id = $1 AND course_id = $2"#, 
  tutor_id as i32, course_id as i32)
  .execute(db_pool)
  .await?;

   match query_result.rows_affected() {
    0 => Err(EzyTutorError::NotFound(format!("Course id `{course_id}` not found"))),
    _ => Ok(format!("Deleted course `{course_id}`")),
  }
  
} // end fn delete_course_db

