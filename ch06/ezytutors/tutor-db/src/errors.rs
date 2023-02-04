use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;

/// Represents an error ocurred in the application.
#[derive(Debug, Serialize)]
pub enum EzyTutorError {
  /// Error from the database.
  DBError(String),
  /// Error from the Actix framework.
  ActixError(String),
  /// Error from not found resources.
  NotFound(String),
}

/// Represents an error message to be send as a response to the user or client.
#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
  /// Error message.
  error_message: String,
}

impl EzyTutorError {
  /// Sends a user-friendly text message to inform that an error has ocurred.
  fn error_response(&self) -> String {
    match self {
      EzyTutorError::DBError(message) => {
        println!("A database error has ocurred: {message}");
        "Database error".into()
      }
      EzyTutorError::ActixError(message) => {
        println!("An Actix server error has ocurrerd: {message}");
        "Internal server error".to_string()
      }
      EzyTutorError::NotFound(message) => {
        println!("A not found error has ocurred: {message}");
        message.into()
      }
    }
  } // end fn error_response()
}

impl actix_web::error::ResponseError for EzyTutorError {
  fn status_code(&self) -> actix_web::http::StatusCode {
    // Specify the HTTP status code for the response message.
    match self {
      EzyTutorError::DBError(_message) | EzyTutorError::ActixError(_message) => {
        StatusCode::INTERNAL_SERVER_ERROR
      }
      EzyTutorError::NotFound(_message) => StatusCode::NOT_FOUND,
    }
  }

  fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
    // Build the body for the response using the status code and the error message.
    HttpResponse::build(self.status_code()).json(MyErrorResponse {
      error_message: self.error_response(),
    })
  }
}

impl std::fmt::Display for EzyTutorError {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      Self::DBError(message) => write!(f, "EzyTutorError::DBError : {message}"),
      Self::ActixError(message) => write!(f, "EzyTutorError::ActixError : {message}"),
      Self::NotFound(message) => write!(f, "EzyTutorError::NotFound : {message}"),
    }
  }
}

impl From<actix_web::error::Error> for EzyTutorError {
  fn from(value: actix_web::error::Error) -> Self {
    EzyTutorError::ActixError(value.to_string())
  }
}

impl From<sqlx::error::Error> for EzyTutorError {
  fn from(value: sqlx::error::Error) -> Self {
    EzyTutorError::DBError(value.to_string())
  }
}
