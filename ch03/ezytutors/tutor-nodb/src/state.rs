use std::sync::Mutex;

/// Represents a group of attributes showing several states of the application.
pub struct AppState {
  /// Health check status.
  pub health_check_response: String,
  /// Counter for user visits.
  pub visit_count: Mutex<u32>,
}
