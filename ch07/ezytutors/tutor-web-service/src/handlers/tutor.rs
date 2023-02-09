use crate::db_access::tutor::*;
use crate::errors::EzyTutorError;
use crate::models::tutor::*;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

/// Gets the entire collection of tutors.
///
/// # Arguments
///
/// * `app_state` - Container of the application state.
pub async fn get_all_tutors(
  app_state: web::Data<AppState>
) -> Result<HttpResponse, EzyTutorError> {
  get_all_tutors_db(&app_state.db)
    .await
    .map(|tutors| HttpResponse::Ok().json(tutors))
} // end fn get_all_tutors()

/// Get the tutor details for the given tutor identifier (ID).
///
/// # Arguments
///
/// * `app_state` - Container of the application state.
/// * `params` - Collection of HTTP query parameters.
pub async fn get_tutor_details(
  app_state: web::Data<AppState>,
  params: web::Path<(u32,)>,
) -> Result<HttpResponse, EzyTutorError> {
  let (tutor_id,) = params.into_inner();

  get_tutor_details_db(&app_state.db, tutor_id)
    .await
    .map(|tutors| HttpResponse::Ok().json(tutors))
} // end fn get_tutor_details()

/// Handler for creating a new tutor.
///
/// # Arguments
///
/// * `app_state` - Container of the application state.
/// * `course` - New tutor to create comming from the HTTP request.
pub async fn post_new_tutor(
  app_state: web::Data<AppState>,
  tutor: web::Json<CreateTutor>,
) -> Result<HttpResponse, EzyTutorError> {
  post_new_tutor_db(&app_state.db, CreateTutor::from(tutor))
    .await
    .map(|tutors| HttpResponse::Ok().json(tutors))
} // end fn post_new_tutor()

/// Handler for updating an existing tutor.
///
/// # Arguments
///
/// * `app_state` - Container of the application state.
/// * `tutor` - Tutor data to be updated.
/// * `params` - Collection of HTTP query parameters.
pub async fn update_tutor_details(
  app_state: web::Data<AppState>,
  tutor: web::Json<UpdateTutor>,
  params: web::Path<(u32,)>,
) -> Result<HttpResponse, EzyTutorError> {
  let (tutor_id,) = params.into_inner();

  update_tutor_details_db(&app_state.db, tutor.into(), tutor_id)
    .await
    .map(|tutors| HttpResponse::Ok().json(tutors))
} // end fn update_tutor_details()

/// Handler for deleting an existing tutor.
///
/// # Arguments
///
/// * `app_state` - Container of the application state.
/// * `params` - Collection of HTTP query parameters.
pub async fn delete_tutor(
  app_state: web::Data<AppState>,
  params: web::Path<(u32,)>,
) -> Result<HttpResponse, EzyTutorError> {
  let (tutor_id,) = params.into_inner();

  delete_tutor_db(&app_state.db, tutor_id)
    .await
    .map(|t| HttpResponse::Ok().json(t))
} // end fn delete_tutor()

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::http::StatusCode;
  use actix_web::ResponseError;
  use dotenvy::dotenv;
  use sqlx::postgres::PgPool;
  use std::env;
  use std::sync::Mutex;

  #[actix_rt::test]
  async fn test_get_all_tutors_success() {
    dotenv().ok();
    let database_url =
      env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db: pool,
    });

    let response = get_all_tutors(app_state).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
  } // end fn test_get_all_tutors_success()

  #[actix_rt::test]
  async fn test_get_tutor_detail_success() {
    dotenv().ok();
    let database_url =
      env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db: pool,
    });

    let params: web::Path<(u32,)> = web::Path::from((3,));
    let response = get_tutor_details(app_state, params).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
  } // end fn test_get_tutor_detail_success()

  #[actix_rt::test]
  async fn test_post_tutor_success() {
    dotenv().ok();
    let database_url =
      env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db: pool,
    });
    let new_tutor = CreateTutor {
      name: "Third tutor".into(),
      pic_url: "http://tutor.s3.com/ssdfds".into(),
      profile: "Experienced tutor in Statistics".into(),
    };

    let tutor_param = web::Json(new_tutor);
    let response = post_new_tutor(app_state, tutor_param).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
  } // end fn test_post_tutor_success()

  #[actix_rt::test]
  #[ignore = "execute after successful update"]
  async fn test_delete_tutor_success() {
    dotenv().ok();
    let database_url =
      env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db: pool,
    });

    let params: web::Path<(u32,)> = web::Path::from((2,));
    let response = delete_tutor(app_state, params).await;

    match response {
      Ok(res) => assert_eq!(res.status(), StatusCode::OK),
      Err(_) => assert!(response.is_ok()),
    }
  } // end fn test_delete_tutor_success()

  #[actix_rt::test]
  #[ignore = "execute after successful failure"]
  async fn test_delete_tutor_failure() {
    dotenv().ok();
    let database_url =
      env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db: pool,
    });

    let params: web::Path<(u32,)> = web::Path::from((1000,));
    let response = delete_tutor(app_state, params).await;

    match response {
      Err(error) => assert_eq!(error.status_code(), StatusCode::NOT_FOUND),
      Ok(_) => assert!(response.is_err()),
    }
  } // end fn test_delete_tutor_failure()
}
