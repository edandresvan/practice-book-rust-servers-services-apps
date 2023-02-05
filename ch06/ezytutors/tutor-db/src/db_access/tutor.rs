use sqlx::postgres::PgPool;

use crate::errors::EzyTutorError;
use crate::models::tutor::*;

/// Gets all tutors.
///
/// # Arguments
///
/// * `db_pool`: Database connection pool.
pub async fn get_all_tutors_db(db_pool: &PgPool) -> Result<Vec<Tutor>, EzyTutorError> {
  // Prepare the SQL select statement

  let tutors: Vec<Tutor> = sqlx::query_as!(
    Tutor,
    r#"SELECT tutor_id, tutor_name as name, 
    tutor_pic_url as pic_url, tutor_profile as profile 
    FROM tutor_ch06"#
  )
  .fetch_all(db_pool)
  .await?;

  match tutors.len() {
    n if n > 0 => Ok(tutors),
    0 => Err(EzyTutorError::NotFound(format!("There are not tutors."))),
    _ => Err(EzyTutorError::DBError(format!(
      "Whoa!, tutors.len() is negative!"
    ))),
  }
} // end fn get_all_tutors_db()

/// Gets the tutor for the given tutor and course identifiers.
///
/// # Arguments
///
/// * `db_pool`: Database connection pool.
/// * `tutor_id`: Unique identifier (ID) of the tutor.
pub async fn get_course_details_db(
  db_pool: &PgPool,
  tutor_id: u32,
) -> Result<Vec<Tutor>, EzyTutorError> {
  // Prepare the SQL SELECT statement
  let tutors: Vec<Tutor> = sqlx::query!(
    r#"SELECT tutor_id, tutor_name, 
    tutor_pic_url, tutor_profile 
    FROM tutor_ch06 
    WHERE tutor_id = $1"#,
    tutor_id as i32
  )
  .fetch_all(db_pool)
  .await?
  .iter()
  .map(|tutor_row| Tutor {
    tutor_id: tutor_row.tutor_id,
    name: tutor_row.tutor_name.clone(),
    pic_url: tutor_row.tutor_pic_url.clone(),
    profile: tutor_row.tutor_profile.clone(),
  })
  .collect();

  if !tutors.is_empty() {
    Ok(tutors)
  } else {
    Err(EzyTutorError::NotFound(format!("Tutor id `{tutor_id}`.")))
  }
} // end fn get_course_details_db

pub async fn post_new_tutor_db(
  db_pool: &PgPool,
  tutor: CreateTutor,
) -> Result<Vec<Tutor>, EzyTutorError> {
  // Prepare the SQL insert statement
  let tutors: Vec<Tutor> = sqlx::query_as!(
    Tutor,
    r#"INSERT INTO tutor_ch06 
    (tutor_name, tutor_pic_url, tutor_profile) 
    VALUES ($1, $2, $3) 
    RETURNING tutor_id, tutor_name as name, tutor_pic_url as pic_url, tutor_profile as profile"#,
    tutor.name,
    tutor.pic_url,
    tutor.profile
  )
  .fetch_all(db_pool)
  .await?;

  Ok(tutors)
} // end fn post_new_tutor_db()
