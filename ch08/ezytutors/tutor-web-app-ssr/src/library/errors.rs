use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;

/// Represents an error in the Ezy System.
#[derive(Debug, Serialize)]
pub enum EzyTutorError {
  /// Represents a database error.
  DBError(String),
  /// Represents an Actix error.
  ActixError(String),
  /// Represents a not found error.
  NotFound(String),
  /// Represents a Tera error.
  TeraError(String),
} // end enum EzyTutorError

/// Represents an error message for the user.
#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
  /// Error message.
  pub error_message: String,
} // end struct MyErrorResponse

// Allow the EzyError to behave like an standard error, and
// also allow Actix to convert the error into an HTTP response.
impl std::error::Error for EzyTutorError {}

// Give a response message according to the error type.
impl EzyTutorError {
  fn error_response(&self) -> String {
    match self {
      EzyTutorError::DBError(message) => {
        println!("A database error has ocurred: {message}");
        format!("A database error has ocurred: {message}")
      }
      EzyTutorError::ActixError(message) => {
        println!("An Actix server error has ocurrerd: {message}");
        format!("Internal server error: {message}")
      }
      EzyTutorError::NotFound(message) => {
        println!("A not found error has ocurred: {message}");
        format!("A not found error has ocurred: {message}")
      }
      EzyTutorError::TeraError(message) => {
        println!("A Terra error has ocurred: {message}");
        format!("A Terra error has ocurred: {message}")
      }
    }
  } // end fn error_response()
} // end impl EzyTutorError

impl actix_web::error::ResponseError for EzyTutorError {
  fn status_code(&self) -> StatusCode {
    match self {
      EzyTutorError::DBError(_)
      | EzyTutorError::ActixError(_)
      | EzyTutorError::TeraError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      EzyTutorError::NotFound(_) => StatusCode::NOT_FOUND,
    }
  } // end fn status_code()

  fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
    HttpResponse::build(self.status_code()).json(MyErrorResponse {
      error_message: self.error_response(),
    })
  } // end fn error_response()
} // end impl actix_web::error::ResponseError

// Allow easy display of the error.
impl std::fmt::Display for EzyTutorError {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      Self::DBError(message) => write!(f, "EzyTutorError::DBError : {message}"),
      Self::ActixError(message) => write!(f, "EzyTutorError::ActixError : {message}"),
      Self::NotFound(message) => write!(f, "EzyTutorError::NotFound : {message}"),
      Self::TeraError(message) => write!(f, "EzyTutorError::TeraError : {message}"),
    }
  } // end fn fmt()
} // end impl std::fmt::Display

// Allow the conversion from EzyError into an Actix error.
impl From<actix_web::error::Error> for EzyTutorError {
  fn from(value: actix_web::error::Error) -> Self {
    EzyTutorError::ActixError(value.to_string())
  } // end fn from()
} // impl From <actix_web::error::Error>

// Allow the conversion from EzyTutor::DBError into a SQLx error.
impl From<sqlx::error::Error> for EzyTutorError {
  fn from(value: sqlx::error::Error) -> Self {
    EzyTutorError::DBError(value.to_string())
  } // end fn from()
} // end impl From<sqlx::error::Error>
