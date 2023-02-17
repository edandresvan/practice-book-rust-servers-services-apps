use crate::errors::EzyTutorError;
use crate::models::*;
use sqlx::postgres::PgPool;

/// Gets the user info using the given username.
///
/// # Arguments
///
/// * `db_pool`: Database connection pool.
/// * `username`: Username of the user.
pub async fn get_user_record_db(
  db_pool: &PgPool,
  username: &str,
) -> Result<User, EzyTutorError> {
  // Preparte the SELECT statement
  let user_row = sqlx::query_as!(
    User,
    r#"SELECT * 
  FROM ezyweb_user 
  WHERE username = $1"#,
    username
  )
  .fetch_optional(db_pool)
  .await?;

  if let Some(user) = user_row {
    Ok(user)
  } else {
    Err(EzyTutorError::NotFound(format!(
      "Username `{username}` not found."
    )))
  }
} // end fn get_user_record_db()

/// Creates (inserts) the given user into the database.
///
/// # Arguments
///
/// * `db_pool`: Database connection pool.
/// * `user`: User to be created.
pub async fn post_new_user_db(
  db_pool: &PgPool,
  user: User,
) -> Result<User, EzyTutorError> {
  let created_user: User = sqlx::query_as!(
    User,
    r#"INSERT INTO ezyweb_user 
  (username, tutor_id, user_password) 
  VALUES ($1, $2, $3) 
  RETURNING username, tutor_id, user_password
  "#,
    user.username,
    user.tutor_id,
    user.user_password
  )
  .fetch_one(db_pool)
  .await?;

  Ok(created_user)
} // end fn fn post_new_user_db()
