use crate::models::Course;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

use crate::errors::EzyTutorError;

/// Gets all courses for the given tutor.
///
/// # Arguments
///
/// * `db_pool`: Database connection pool.
/// * `tutor_id`: Unique identifier (ID) of the tutor.
pub async fn get_courses_for_tutor_db(
  db_pool: &PgPool,
  tutor_id: i32,
) -> Result<Vec<Course>, EzyTutorError> {
  // SQL query to retrieve course rows
  let course_rows = sqlx::query!(
    r#"SELECT course_id, tutor_id, course_name, posted_time FROM course_ch05 WHERE tutor_id = $1"#,
    tutor_id
  ).fetch_all(db_pool).await?;

  // Create a collection of courses object from the courses rows
  let courses: Vec<Course> = course_rows
    .iter()
    .map(|course_row| Course {
      course_id: course_row.course_id as u32,
      tutor_id: course_row.tutor_id as u32,
      course_name: course_row.course_name.clone(),
      posted_time: Some(NaiveDateTime::from(course_row.posted_time.unwrap())),
    })
    .collect();

  if courses.len() > 0 {
    return Ok(courses);
  } else {
    return Err(EzyTutorError::NotFound(
      "Courses not found for tutor.".to_string(),
    ));
  }
} // end fn get_courses_for_tutor_db()

/// Gets all courses for the given tutor and course identifiers.
///
/// # Arguments
///
/// * `db_pool`: Database connection pool.
/// * `tutor_id`: Unique identifier (ID) of the tutor.
/// * `course_id`: Unique identifier (ID) of the course.
pub async fn get_course_details_db(
  db_pool: &PgPool,
  tutor_id: i32,
  course_id: i32,
) -> Result<Vec<Course>, EzyTutorError> {
  // SQL query to retrieve course rows
  let rows = sqlx::query!("SELECT tutor_id, course_id, course_name, posted_time FROM course_ch05 WHERE tutor_id = $1 AND course_id = $2", tutor_id, course_id).fetch_all(db_pool).await;

  match rows {
    Ok(course_rows) => {
      // Create a collection of courses object from the courses rows
      let courses: Vec<Course> = course_rows
        .iter()
        .map(|course_row| Course {
          course_id: course_row.course_id as u32,
          tutor_id: course_row.tutor_id as u32,
          course_name: course_row.course_name.clone(),
          posted_time: Some(NaiveDateTime::from(course_row.posted_time.unwrap())),
        })
        .collect();

      if courses.len() > 0 {
        return Ok(courses);
      } else {
        return Err(EzyTutorError::NotFound("Course id not found.".to_string()));
      }
    }
    Err(err) => {
      return Err(EzyTutorError::DBError(err.to_string()));
    }
  }
} // end fn get_course_details_db()

/// Creates (inserts) the given course into the database.
///
/// # Arguments
///
/// * `db_pool`: Database connection pool.
/// * `new_course`: Course to be created.
pub async fn post_new_course_db(
  db_pool: &PgPool,
  new_course: Course,
) -> Vec<Course> {
  // Prepare the SQL insert
  let course_rows = sqlx::query!("INSERT INTO course_ch05 (course_id, tutor_id, course_name) VALUES ($1, $2, $3) RETURNING course_id, tutor_id, course_name, posted_time", new_course.course_id as i32, new_course.tutor_id as i32, new_course.course_name).fetch_all(db_pool).await.unwrap();

  // Create a collection of courses object from the courses rows
  course_rows
    .iter()
    .map(|course_row| Course {
      course_id: course_row.course_id as u32,
      tutor_id: course_row.tutor_id as u32,
      course_name: course_row.course_name.clone(),
      posted_time: Some(NaiveDateTime::from(course_row.posted_time.unwrap())),
    })
    .collect()
}
