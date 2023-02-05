use crate::db_access::course::*;
use crate::models::course::{CreateCourse, UpdateCourse};

use actix_web::{web, HttpResponse};

use crate::errors::EzyTutorError;

use crate::state::AppState;

/// Gets the collection of courses belonging to the give tutor ID.
///
/// # Arguments
///
/// * `app_state` - Container of the application state.
/// * `params` - Collection of HTTP query parameters.
pub async fn get_courses_for_tutor(
  app_state: web::Data<AppState>,
  params: web::Path<u32>,
) -> Result<HttpResponse, EzyTutorError> {
  let tutor_id = params.into_inner();

  get_courses_for_tutor_db(&app_state.db, tutor_id)
    .await
    .map(|courses| HttpResponse::Ok().json(courses))
} // end fn get_courses_for_tutor()

/// Get the course datails for the given tutor and course identifiers (IDs).
///
/// # Arguments
///
/// * `app_state` - Container of the application state.
/// * `params` - Collection of HTTP query parameters.
pub async fn get_course_details(
  app_state: web::Data<AppState>,
  params: web::Path<(u32, u32)>,
) -> Result<HttpResponse, EzyTutorError> {
  let (tutor_id, course_id) = params.into_inner();

  let response: Result<HttpResponse, EzyTutorError> =
    get_course_details_db(&app_state.db, tutor_id, course_id)
      .await
      .map(|courses| HttpResponse::Ok().json(courses));

  response
} // end fn get_course_details()

/// Handler for creating a new course.
///
/// # Arguments
///
/// * `new_course` - New course to create comming from the HTTP request.
/// * `app_state` - Container of the application state.
pub async fn post_new_course(
  course: web::Json<CreateCourse>,
  app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> {
  let response: Result<HttpResponse, EzyTutorError> =
    post_new_course_db(&app_state.db, course.into())
      .await
      .map(|courses| HttpResponse::Ok().json(courses));
  response
} // end fn post_new_course()

/// Handler for updating an existing course.
///
/// # Arguments
///
/// * `app_state` - Container of the application state.
/// * `course` - Course data to be updated.
/// * `params` - Collection of HTTP query parameters.
pub async fn update_course_details(
  app_state: web::Data<AppState>,
  course: web::Json<UpdateCourse>,
  path: web::Path<(u32, u32)>,
) -> Result<HttpResponse, EzyTutorError> {
  let (tutor_id, course_id) = path.into_inner();

  update_course_details_db(&app_state.db, tutor_id, course_id, course.into())
    .await
    .map(|courses| HttpResponse::Ok().json(courses))
}

/// Handler for deleting an existing course.
///
/// # Arguments
///
/// * `app_state` - Container of the application state.
/// * `params` - Collection of HTTP query parameters.
pub async fn delete_course(
  app_state: web::Data<AppState>,
  path: web::Path<(u32, u32)>,
) -> Result<HttpResponse, EzyTutorError> {
  let (tutor_id, course_id) = path.into_inner();

  // let response = delete_course_db(&app_state.db, tutor_id, course_id).await?;

  // Ok(HttpResponse::Ok().json(response))

  delete_course_db(&app_state.db, tutor_id, course_id)
    .await
    .map(|resp| HttpResponse::Ok().json(resp))
} // end fn delete_course()

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
  async fn test_get_all_courses_success() {
    dotenv().ok();

    let database_url: String =
      env::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file.");

    let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

    let app_state: web::Data<AppState> = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db: pool,
    });

    let tutor_id: web::Path<u32> = web::Path::from(1);
    let response: HttpResponse =
      get_courses_for_tutor(app_state, tutor_id).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
  } // end fn test_get_all_courses_success()

  #[actix_rt::test]
  async fn test_get_course_detail_success() {
    dotenv().ok();

    let database_url: String =
      env::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file.");

    let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

    let app_state: web::Data<AppState> = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db: pool,
    });

    let params: web::Path<(u32, u32)> = web::Path::from((1, 2));
    let response: HttpResponse = get_course_details(app_state, params).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
  } // end fn test_get_course_detail_success()

  #[actix_rt::test]
  async fn test_get_course_detail_failure() {
    dotenv().ok();

    let database_url: String =
      env::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file.");

    let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

    let app_state: web::Data<AppState> = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db: pool,
    });

    let params: web::Path<(u32, u32)> = web::Path::from((1, 20000));

    let response = get_course_details(app_state, params).await;

    assert_eq!(
      response.as_ref().unwrap_err().status_code(),
      StatusCode::NOT_FOUND
    );

    match &response {
      Err(error) => assert_eq!(error.status_code(), StatusCode::NOT_FOUND),
      Ok(_) => assert!(response.is_err()),
    }
  } // end fn test_get_course_detail_failure()

  #[actix_rt::test]
  async fn test_post_course_success() {
    dotenv().ok();

    let database_url: String =
      env::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file.");

    let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

    let app_state: web::Data<AppState> = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db: pool,
    });

    let new_course = CreateCourse {
      tutor_id: 1,
      name: "Third course".to_string(),
      description: Some("This is a test course".to_string()),
      level: Some("Beginner".to_string()),
      language: Some("English".to_string()),
      format: None,
      duration: None,
      price: None,
      structure: None,
    };

    let course_json_param: web::Json<CreateCourse> = web::Json(new_course);
    let response: HttpResponse =
      post_new_course(course_json_param, app_state).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
  } // end fn test_post_course_success()

  #[actix_rt::test]
  async fn test_update_course_success() {
    dotenv().ok();

    let database_url: String =
      env::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file.");

    let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

    let app_state: web::Data<AppState> = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db: pool,
    });

    let updated_course = UpdateCourse {
      name: Some("Course name changed".into()),
      description: Some("This is another test course now".to_string()),
      level: Some("Intermediate".to_string()),
      language: Some("German".to_string()),
      price: None,
      format: None,
      duration: None,
      structure: None,
    };

    let params: web::Path<(u32, u32)> = web::Path::from((1, 2));

    let course_param: web::Json<UpdateCourse> = web::Json(updated_course);

    let response = update_course_details(app_state, course_param, params)
      .await
      .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
  } // end fn test_update_course_success()

  #[actix_rt::test]
  #[ignore = "execute after successful update"]
  async fn test_delete_course_success() {
    dotenv().ok();

    let database_url: String =
      env::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file.");

    let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

    let app_state: web::Data<AppState> = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db: pool,
    });

    let parameters: web::Path<(u32, u32)> = web::Path::from((1, 16));

    let response = delete_course(app_state, parameters).await;

    assert!(response.is_ok());
    assert_eq!(response.unwrap().status(), StatusCode::OK);
  } // end fn test_delete_course_success()

  #[actix_rt::test]
  // #[ignore = "execute after successful update"]
  async fn test_delete_course_failure() {
    dotenv().ok();

    let database_url: String =
      env::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file.");

    let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

    let app_state: web::Data<AppState> = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db: pool,
    });

    let parameters: web::Path<(u32, u32)> = web::Path::from((1, 16));

    let response = delete_course(app_state, parameters).await;

    assert_eq!(
      response.as_ref().unwrap_err().status_code(),
      StatusCode::NOT_FOUND
    );

    match response {
      Err(error) => assert_eq!(error.status_code(), StatusCode::NOT_FOUND),
      Ok(_) => assert!(response.is_err()),
    }
  } // end fn test_delete_course_failure()
}
