use sqlx::postgres::{PgPool, PgQueryResult};

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
    0 => Err(EzyTutorError::NotFound("There are not tutors.".to_string())),
    _ => Err(EzyTutorError::DBError(
      "Whoa!, tutors.len() is negative!"
    .to_string())),
  }
} // end fn get_all_tutors_db()

/// Gets the tutor for the given tutor and course identifiers.
///
/// # Arguments
///
/// * `db_pool`: Database connection pool.
/// * `tutor_id`: Unique identifier (ID) of the tutor.
pub async fn get_tutor_details_db(
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
    Err(EzyTutorError::NotFound(format!("Tutor id `{tutor_id}` not found.")))
  }
} // end fn get_course_details_db

pub async fn post_new_tutor_db(
  db_pool: &PgPool,
  tutor: CreateTutor,
) -> Result<Vec<Tutor>, EzyTutorError> {
  // Prepare the SQL insert statement
  let tutor = sqlx::query_as!(
    Tutor,
    r#"INSERT INTO tutor_ch06 
    (tutor_name, tutor_pic_url, tutor_profile) 
    VALUES ($1, $2, $3) 
    RETURNING tutor_id, tutor_name as name, tutor_pic_url as pic_url, tutor_profile as profile"#,
    tutor.name,
    tutor.pic_url,
    tutor.profile
  )
  .fetch_one(db_pool)
  .await?;

  Ok(vec![tutor])
} // end fn post_new_tutor_db()

/// Updates the given tutor parameteres in the database.
///
/// # Arguments
///
/// * `db_pool`: Database connection pool.
/// * `tutor`: Tutor to be updated.
/// * `tutor_id`: Tutor ID.
pub async fn update_tutor_details_db(db_pool: &PgPool, tutor: UpdateTutor, tutor_id: u32, ) -> Result<Vec<Tutor>, EzyTutorError> {

  let mut existing_tutor = sqlx::query_as!(Tutor,
  r#"SELECT tutor_id, tutor_name as "name", tutor_pic_url as "pic_url", tutor_profile as "profile" 
  FROM tutor_ch06 
  WHERE tutor_id = $1"#,
tutor_id as i32).fetch_one(db_pool).await.map_err(|_error| EzyTutorError::NotFound(format!("Tutor id `{tutor_id}` not found.")))?;

    // Change the existing tutor fields if there is data
    if tutor.name.is_some() {
      existing_tutor.name = tutor.name.unwrap();
    }

    if tutor.pic_url.is_some() {
      existing_tutor.pic_url = tutor.pic_url.unwrap();
    }

    if tutor.profile.is_some() {
      existing_tutor.profile = tutor.profile.unwrap();
    }

    // Prepare the SQL update statement
    let updated_tutor_result = sqlx::query_as!(Tutor, 
      r#"UPDATE tutor_ch06 SET 
      tutor_name = $1, tutor_pic_url = $2, tutor_profile = $3 
      WHERE tutor_id= $4 
      RETURNING 
      tutor_id, tutor_name as "name", tutor_pic_url as "pic_url", tutor_profile as "profile" "#,
      existing_tutor.name, existing_tutor.pic_url, existing_tutor.profile, tutor_id as i32)
    .fetch_one(db_pool)
    .await;

    match updated_tutor_result {
      Ok(up_tutor) => Ok(vec![up_tutor]),
      Err(_error) => Err(EzyTutorError::NotFound(format!("Tutor id `{tutor_id}` not found"))),
    }

} // end fn update_tutor_details_db()


/// Removes (deletes) the given tutor parameteres from the database.
///
/// # Arguments
///
/// * `db_pool`: Database connection pool.
/// * `tutor_id`: Tutor ID.
pub async fn delete_tutor_db(db_pool: &PgPool, tutor_id: u32) -> Result<String, EzyTutorError>{
  // Prepare the SQL delete statement
  let query_result: PgQueryResult = sqlx::query!(
    r#"DELETE FROM tutor_ch06 
    WHERE tutor_id = $1 "#, tutor_id as i32
  ).execute(db_pool)
  .await?;

  match query_result.rows_affected() {
    0 => Err(EzyTutorError::NotFound(format!("Tutor id `{tutor_id}` not found"))),
    _ => Ok(format!("Deleted tutor `{tutor_id}`")),
  }
} // end fn delete_tutor_db() 