use actix_web::web;
use serde::{Deserialize, Serialize};

/// Represents a course dictated by a tutor.
#[derive(Serialize, Deserialize, Debug, Clone)]pub struct Course {
  /// Unique identifier (ID) of the course.
  pub course_id: i32,
  /// Unique identifier (ID) of the tutor.
  pub tutor_id: i32,
  /// Name of the course.
  pub name: String,
  /// Timestamp when the course was created.
  pub posted_time: String,
  /// Textual description of the course.
  pub description: String,
  /// Format of course delivery.
  pub format: String,
  /// Document or brochure describing the course.
  pub structure: Option<String>,
  /// Length of the course.
  pub duration: String,
  /// Course price in U.S. dollars.
  pub price: Option<i32>,
  /// Course language.
  pub language: Option<String>,
  /// Level of the student of the course.
  pub level: Option<String>,
} // end struct Course

impl From<web::Json<Course>> for Course {
  fn from(value: web::Json<Course>) -> Self {
    Self {
      course_id: value.course_id,
      tutor_id: value.tutor_id,
      name: value.name.clone(),
      posted_time: value.posted_time.clone(),
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

/// Represents a course for the CREATE action.
#[derive(Deserialize, Debug, Clone)]
pub struct CourseCreate {
  /// Name of the course.
  pub name: String,
  /// Textual description of the course.
  pub description: String,
  /// Format of course delivery.
  pub format: String,
  /// Document or brochure describing the course.
  pub structure: Option<String>,
  /// Length of the course.
  pub duration: String,
  /// Course price in U.S. dollars.
  pub price: Option<i32>,
  /// Course language.
  pub language: Option<String>,
  /// Level of the student of the course.
  pub level: Option<String>,
} // end struct CourseCreate

/// Represents a course for the UPDATE action.
#[derive(Deserialize, Debug, Clone)]
pub struct CourseUpdate {
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
} // end struct CourseUpdate

