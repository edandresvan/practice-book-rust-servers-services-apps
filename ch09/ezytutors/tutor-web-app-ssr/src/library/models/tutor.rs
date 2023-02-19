use serde::{Deserialize, Serialize};

/// Represents info about a tutor for the registration form.
#[derive(Debug, Serialize, Deserialize)]
pub struct TutorRegisterForm {
  /// Username of the tutor.
  pub username: String,
  /// Password of the tutor.
  pub password: String,
  /// Confirmation of the password of the tutor.
  pub confirmation: String,
  /// Name of the tutor.
  pub name: String,
  /// URL of the profile image of the tutor.
  pub image_url: String,
  /// Profile bio of the tutor.
  pub profile: String,
} // end struct TutorRegisterForm

/// Represents a tutor from the backend web service.
#[derive(Debug, Serialize, Deserialize)]
pub struct TutorResponse {
  /// Unique identifier (ID) of the tutor.
  pub tutor_id: i32,
  /// Name of the tutor.
  pub name: String,
  /// URL of the profile image of the tutor.
  pub pic_url: String,
  /// Profile bio of the tutor.
  pub profile: String,
} // end struct TutorResponse

/// Represents the info of a Tutor logging in the web app. 
#[derive(Debug, Serialize, Deserialize)]
pub struct TutorSigninForm {
  /// Username of the tutor user.
  pub username: String,
  /// Password of the tutor user.
  pub password: String,
} // end struct TutorSigninForm

