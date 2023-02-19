use crate::models::course::*;
use crate::state::AppState;
use actix_web::{web, Error as ActixError, HttpResponse, Result};
use serde_json::json;

/// Register a new user from the given parameters.
///
/// # Arguments
///
/// * `app_state`: Container of the application state.
/// * `template_engine`:  Tera engine object to create the HTML page.
/// * `path`: Collection of path parameters.
/// * `params` - Collection of HTTP query parameters.
pub async fn handle_insert_course(
  _app_state: web::Data<AppState>,
  _template_engine: web::Data<tera::Tera>,
  path: web::Path<i32>,
  params: web::Json<CourseCreate>,
) -> Result<HttpResponse, ActixError> {
  let tutor_id = path.into_inner();
  // Create an object from the HTTP params.
  let course_json = json!({
    "tutor_id": tutor_id,
    "name": &params.name,
    "description": &params.description,
    "format": &params.format,
    "structure": &params.structure,
    "duration": &params.duration,
    "price": &params.price,
    "language": &params.language,
    "level": &params.level
  });

  // Try to connect and sent a POST request to create a new course.
  let awc_client = awc::Client::new();
  let response_raw = awc_client
    .post("http://localhost:3000/courses")
    .send_json(&course_json)
    .await
    .unwrap()
    .body()
    .await?;

  // Convert the raw response into a Course object by using the impl From trait.
  let courses: Vec<Course> = serde_json::from_str(std::str::from_utf8(&response_raw)?)?;

  Ok(HttpResponse::Ok().json(courses))
} // end fn handle_insert_course()

/// Register a new user from the given parameters.
///
/// # Arguments
///
/// * `app_state`: Container of the application state.
/// * `template_engine`:  Tera engine object to create the HTML page.
pub async fn handle_update_course(
  app_state: web::Data<AppState>,
  template_engine: web::Data<tera::Tera>,
) -> Result<HttpResponse, ActixError> {
  Ok(HttpResponse::Ok().body("Got update"))
} // end fn handle_update_course()

/// Register a new user from the given parameters.
///
/// # Arguments
///
/// * `app_state`: Container of the application state.
/// * `template_engine`:  Tera engine object to create the HTML page.
pub async fn handle_delete_course(
  app_state: web::Data<AppState>,
  template_engine: web::Data<tera::Tera>,
) -> Result<HttpResponse, ActixError> {
  Ok(HttpResponse::Ok().body("Got delete"))
} // end fn handle_delete_course()
