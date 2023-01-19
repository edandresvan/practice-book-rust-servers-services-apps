use std::sync::Mutex;
use crate::models::Course;

/// Represents a group of attributes showing several states of the application.
pub struct AppState {
  /// Health check status.
  pub health_check_response: String,
  /// Counter for user visits.
  pub visit_count: Mutex<u32>,
  /// Collection of registered courses.
  pub courses: Mutex<Vec<Course>>,
}
