
use serde::{Deserialize, Serialize};

/// Represents info about user credentials for the authentication.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  /// Username.
  pub username: String,
  /// Unique identifier (ID) of the tutor.
  pub tutor_id: Option<i32>,
  /// Password of the user.
  pub user_password: String,
} // end struct User

