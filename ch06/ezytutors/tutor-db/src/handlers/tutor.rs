use crate::db_access::tutor::*;
use crate::errors::EzyTutorError;
use crate::models::tutor::*;
use crate::state::AppState;
use actix_web::{http, web, HttpResponse};

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

  update_tutor_details_db(&app_state.db, tutor_id, tutor)
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
