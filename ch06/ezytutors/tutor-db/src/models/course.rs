use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Represents a course dictated by a tutor.
#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
  /// Unique identifier (ID) of the course.
  pub course_id: i32,
  /// Unique identifier (ID) of the tutor.
  pub tutor_id: i32,
  /// Name of the course.
  #[sqlx(rename = "course_name")]
  pub name: String,
  /// Timestamp when the course was created.
  pub posted_time: Option<NaiveDateTime>,
  /// Textual description of the course.
  #[sqlx(rename = "course_description")]
  pub description: Option<String>,
  /// Format of course delivery.
  #[sqlx(rename = "course_format")]
  pub format: Option<String>,
  /// Document or brochure describing the course.
  #[sqlx(rename = "course_estructure")]
  pub structure: Option<String>,
  /// Length of the course.
  #[sqlx(rename = "course_duration")]
  pub duration: Option<String>,
  /// Course price in U.S. dollars.
  #[sqlx(rename = "course_price")]
  pub price: Option<i32>,
  /// Course language.
  #[sqlx(rename = "course_language")]
  pub language: Option<String>,
  /// Level of the student of the course.
  #[sqlx(rename = "course_level")]
  pub level: Option<String>,
}

/* impl From<web::Json<Course>> for Course {
  fn from(value: web::Json<Course>) -> Self {
    Self {
      tutor_id: value.tutor_id,
      course_id: value.course_id,
      name: value.name.clone(),
      posted_time: value.posted_time,
    }
  }
} */

/// Represents a course for the CREATE action.
#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
  /// Unique identifier (ID) of the tutor.
  pub tutor_id: i32,
  /// Name of the course.
  pub name: String,
  /// Textual description of the course.
  pub description: Option<String>,
  /// Format of course delivery.
  pub format: Option<String>,
  /// Document or brochure describing the course.
  pub structure: Option<String>,
  /// Length of the course.
  pub duration: Option<String>,
  /// Course price in U.S. dollars.
  pub price: Option<i32>,
  /// Course language.
  pub language: Option<String>,
  /// Level of the student of the course.
  pub level: Option<String>,
}

impl From<web::Json<CreateCourse>> for CreateCourse {
  fn from(value: web::Json<CreateCourse>) -> Self {
    Self {
      tutor_id: value.tutor_id,
      name: value.name.clone(),
      description: value.description.clone(),
      format: value.format.clone(),
      structure: value.structure.clone(),
      duration: value.duration.clone(),
      price: value.price,
      language: value.language.clone(),
      level: value.level.clone(),
    }
  }
}

/* impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
  type Error = EzyTutorError;

  fn try_from(value: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
    Ok(Self {
      tutor_id: value.tutor_id,
      name: value.name.clone(),
      description: value.description.clone(),
      format: value.format.clone(),
      structure: value.structure.clone(),
      duration: value.duration.clone(),
      price: value.price,
      language: value.language.clone(),
      level: value.level.clone(),
    })
  }
}
 */

/// Represents a course for the UPDATE action.
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
  /// Name of the course.
  pub name: Option<String>,
  /// Textual description of the course.
  pub description: Option<String>,
  /// Format of course delivery.
  pub format: Option<String>,
  /// Document or brochure describing the course.
  pub structure: Option<String>,
  /// Length of the course.
  pub duration: Option<String>,
  /// Course price in U.S. dollars.
  pub price: Option<i32>,
  /// Course language.
  pub language: Option<String>,
  /// Level of the student of the course.
  pub level: Option<String>,
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
  fn from(value: web::Json<UpdateCourse>) -> Self {
    Self {
      name: value.name.clone(),
      description: value.description.clone(),
      format: value.format.clone(),
      structure: value.structure.clone(),
      duration: value.duration.clone(),
      price: value.price,
      language: value.language.clone(),
      level: value.level.clone(),
    }
  }
}
