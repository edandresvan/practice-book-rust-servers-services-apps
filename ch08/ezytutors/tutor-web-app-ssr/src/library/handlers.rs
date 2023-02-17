use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Result;

use crate::errors::EzyTutorError;

use crate::db_access::*;
use crate::models::TutorRegisterForm;
use crate::models::TutorResponse;
use crate::models::User;
use crate::state::AppState;
use serde_json::json;

/// Handles the request to show the registration form.
pub async fn show_register_form(
  template_engine: web::Data<tera::Tera>
) -> Result<HttpResponse, actix_web::error::Error> {
  let mut context = tera::Context::new();

  context.insert("errors", "");

  context.insert("current_username", "");
  context.insert("current_password", "");
  context.insert("current_password", "");
  context.insert("current_confirmation", "");
  context.insert("current_name", "");
  context.insert("current_image_url", "");
  context.insert("current_profile", "");

  let contents: String = template_engine
    .render("register.html", &context)
    .map_err(|error| EzyTutorError::TeraError(format!("Tera template error: {error}")))?;

  Ok(HttpResponse::Ok().content_type("text/html").body(contents))
} // end fn show_register_form()

/// Handles the requests to process a registration.
pub async fn handle_register(
  template_engine: web::Data<tera::Tera>,
  app_state: web::Data<AppState>,
  params: web::Form<TutorRegisterForm>,
) -> Result<HttpResponse, actix_web::error::Error> {
  let username: &str = params.username.as_str();

  // Check if the password and the confirmation password have the same value
  if params.password != params.confirmation {
    let mut context = tera::Context::new();
    context.insert("errors", "Passwords value do not match");
    context.insert("current_username", &params.username);
    context.insert("current_password", "");
    context.insert("current_confirmation", "");
    context.insert("current_name", &params.name);
    context.insert("current_image_url", &params.image_url);
    context.insert("current_profile", &params.profile);
    let contents: String = template_engine
      .render("register.html", &context)
      .map_err(|error| EzyTutorError::TeraError(format!("Template error: {error:#?}")))?;

    return Ok(HttpResponse::Ok().body(contents));
  }

  // Search for the existing user
  let user_result = get_user_record_db(&app_state.db_pool, username).await;

  match user_result {
    Ok(_user) => show_error_existing_user(&template_engine, &params).await,
    Err(_error) => register_new_user(&app_state, &params).await,
  }
} // end fn handle_register()

async fn register_new_user(
  app_state: &web::Data<AppState>,
  params: &web::Form<TutorRegisterForm>,
) -> Result<HttpResponse, actix_web::error::Error> {
  // Create the tutor in the web service database (back-end)
  let tutor_params = json!({
    "name": &params.name,
    "pic_url": &params.image_url,
    "profile": &params.profile
  });

  let awc_client = awc::Client::new();
  let raw_response = awc_client
    .post("http://localhost:3000/tutors")
    .send_json(&tutor_params)
    .await
    .unwrap()
    .body()
    .await?;

  // Convert the received raw bytes into a collection of tutors
  let tutors: Vec<TutorResponse> =
    serde_json::from_str(std::str::from_utf8(&raw_response)?)?;
  let created_tutor = tutors.get(0).unwrap();

  // Give the password a hash o salt value
  let salt_value = b"mysweetsalt";
  let config = argon2::Config::default();
  let hashed_password =
    argon2::hash_encoded(params.password.clone().as_bytes(), salt_value, &config)
      .unwrap();

  // Create the new user in the database
  let user = User {
    username: params.username.clone(),
    tutor_id: Some(created_tutor.tutor_id),
    user_password: hashed_password,
  };

  let user_created = post_new_user_db(&app_state.db_pool, user).await?;
  // Send a response message
  let contents: String = format!(
    "Congratulations. You have registered successfully with the tutor ID: `{0}`",
    &user_created.tutor_id.unwrap()
  );
  Ok(HttpResponse::Ok().body(contents))
} // end fn register_new_user()

/// Shows a message informing that another user already exists with the specified username in the parameters set.
async fn show_error_existing_user(
  template_engine: &web::Data<tera::Tera>,
  params: &web::Form<TutorRegisterForm>,
) -> Result<HttpResponse, actix_web::error::Error> {
  // The user already exists, so send the params again to the end user
  let mut context = tera::Context::new();
  context.insert(
    "errors",
    format!("Username '{0}' already exists", &params.username).as_str(),
  );
  context.insert("current_username", &params.username.clone());
  context.insert("current_password", "");
  context.insert("current_confirmation", "");
  context.insert("current_name", &params.name);
  context.insert("current_image_url", &params.image_url);
  context.insert("current_profile", &params.profile);

  let contents: String = template_engine
    .render("register.html", &context)
    .map_err(|error| EzyTutorError::TeraError(format!("Template error: {error:#?}")))?;

  Ok(HttpResponse::Ok().body(contents))
} // end show_error_existing_user()
