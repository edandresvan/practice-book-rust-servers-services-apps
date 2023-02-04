use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Represents a course dictated by a tutor.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
  /// Unique identifier (ID) of the tutor.
  pub tutor_id: usize,
  /// Unique identifier (ID) of the course.
  pub course_id: Option<usize>,
  /// Name of the course.
  pub course_name: String,
  /// Timestamp when the course was created.
  pub posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
  fn from(value: web::Json<Course>) -> Self {
    // Converts JSON data into a course
    Self {
      tutor_id: value.tutor_id,
      course_id: value.course_id,
      course_name: value.course_name.clone(),
      posted_time: value.posted_time,
    }
  }
}
