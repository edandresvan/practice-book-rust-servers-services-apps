use sqlx::postgres::PgPool;

/// Represents the application state info.
pub struct AppState {
  /// Database pool for connections.
  pub db_pool: PgPool,
} // end struct AppState
