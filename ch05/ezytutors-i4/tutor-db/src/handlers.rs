use crate::{models::Course, state::AppState};

use actix_web::{web, HttpResponse};

use crate::db_access::*;

use crate::errors::EzyTutorError;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
  let health_check_response = &app_state.health_check_response;

  let mut visit_count = app_state.visit_count.lock().unwrap();

  let response = format!("{} {} times", health_check_response, visit_count);

  *visit_count += 1;

  HttpResponse::Ok().json(&response)
} // end fn health_check_handler()

/// Gets the collection of courses belonging to the give tutor ID.
///
/// # Arguments
///
/// * `app_state` - Container of the application state.
/// * `params` - Collection of HTTP query parameters.
pub async fn get_courses_for_tutor(
  app_state: web::Data<AppState>,
  params: web::Path<usize>,
) -> Result<HttpResponse, EzyTutorError> {
  let tutor_id = params.into_inner() as i32;

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
  params: web::Path<(usize, usize)>,
) -> Result<HttpResponse, EzyTutorError> {
  let (tutor_id, course_id) = params.into_inner();

  let response = get_course_details_db(&app_state.db, tutor_id as i32, course_id as i32)
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
  new_course: web::Json<Course>,
  app_state: web::Data<AppState>,
) -> HttpResponse {
  let courses: Vec<Course> = post_new_course_db(&app_state.db, new_course.into()).await;

  HttpResponse::Ok().json(courses)
} // end fn post_new_course()

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::http::StatusCode;
  use chrono::NaiveDate;
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

    let tutor_id: web::Path<usize> = web::Path::from(1);
    let response: HttpResponse =
      get_courses_for_tutor(app_state, tutor_id).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
  } // end fn test_get_all_courses_success()

  #[actix_rt::test]
  async fn test_get_course_detail() {
    dotenv().ok();

    let database_url: String =
      env::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file.");

    let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

    let app_state: web::Data<AppState> = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      db: pool,
    });

    let params: web::Path<(usize, usize)> = web::Path::from((1, 1));
    let response: HttpResponse = get_course_details(app_state, params).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
  } // end fn test_get_course_detail()

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

    let new_course = Course {
      course_id: 1000,
      tutor_id: 1000,
      course_name: "This is the next course".to_string(),
      posted_time: Some(
        NaiveDate::from_ymd_opt(2020, 9, 17)
          .unwrap()
          .and_hms_opt(14, 01, 11)
          .unwrap(),
      ),
    };

    let course_param = web::Json(new_course);
    let response: HttpResponse = post_new_course(course_param, app_state).await;

    assert_eq!(response.status(), StatusCode::OK);
  }
}
