use actix_web::web;
use serde::{Deserialize, Serialize};

/// Represents a tutor who directs a course.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tutor {
  /// Unique identifier (ID) of the tutor.
  pub tutor_id: i32,
  /// Full name of the tutor.
  pub name: String,
  /// URL of the image profile of the tutor.
  pub pic_url: String,
  /// Brief profile of the tutor.
  pub profile: String,
}

/// Represents a tutor for the CREATE action.
#[derive(Deserialize, Debug, Clone)]
pub struct CreateTutor {
  /// Full name of the tutor.
  pub name: String,
  /// URL of the image profile of the tutor.
  pub pic_url: String,
  /// Brief profile of the tutor.
  pub profile: String,
}

impl From<web::Json<CreateTutor>> for CreateTutor {
  fn from(value: web::Json<CreateTutor>) -> Self {
    Self {
      name: value.name.clone(),
      pic_url: value.pic_url.clone(),
      profile: value.profile.clone(),
    }
  }
}

/// Represents a tutor for the UPDATE action.
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTutor {
  /// Full name of the tutor.
  pub name: Option<String>,
  /// URL of the image profile of the tutor.
  pub pic_url: Option<String>,
  /// Brief profile of the tutor.
  pub profile: Option<String>,
}

impl From<web::Json<UpdateTutor>> for UpdateTutor {
  fn from(value: web::Json<UpdateTutor>) -> Self {
    Self {
      name: value.name.clone(),
      pic_url: value.pic_url.clone(),
      profile: value.profile.clone(),
    }
  }
}
