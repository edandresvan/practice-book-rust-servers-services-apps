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
/// * `path`: Collection of path parameters.
/// * `params` - Collection of HTTP query parameters.
pub async fn handle_update_course(
  _app_state: web::Data<AppState>,
  _template_engine: web::Data<tera::Tera>,
  path: web::Path<(i32, i32)>,
  params: web::Json<CourseUpdate>,
) -> Result<HttpResponse, ActixError> {
  let (tutor_id, course_id) = path.into_inner();

  let course_json = json!({
    "name": &params.name,
    "description": &params.description,
    "format": &params.format,
    "structure": &params.structure,
    "duration": &params.duration,
    "price": &params.price,
    "language": &params.language,
    "level": &params.level,
  });

  let http_client = awc::Client::new();
  let update_url = format!("http://localhost:3000/courses/{tutor_id}/{course_id}");
  

  let response_raw = http_client
    .put(update_url)
    .send_json(&course_json)
    .await
    .unwrap()
    .body()
    .await?;

  let courses: Vec<Course> = serde_json::from_str(std::str::from_utf8(&response_raw)?)?;

  Ok(HttpResponse::Ok().json(courses))
} // end fn handle_update_course()
/// Register a new user from the given parameters.
///
/// # Arguments
///
/// * `app_state`: Container of the application state.
/// * `template_engine`:  Tera engine object to create the HTML page.
/// * `path`: Collection of path parameters.
pub async fn handle_delete_course(
  _app_state: web::Data<AppState>,
  _template_engine: web::Data<tera::Tera>,
  path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, ActixError> {
  let (tutor_id, course_id) = path.into_inner();

  let http_client = awc::Client::new();

  let delete_url = format!("http://localhost:3000/courses/{tutor_id}/{course_id}");

  let response_raw = http_client.delete(delete_url).send().await.unwrap().body().await?;

  let response: String = serde_json::from_str(std::str::from_utf8(&response_raw)?)?;

  Ok(HttpResponse::Ok().body(response))
} // end fn handle_delete_course()
