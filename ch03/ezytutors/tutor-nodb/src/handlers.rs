use crate::models::Course;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use chrono::Utc;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
  let health_check_response = &app_state.health_check_response;
  // Lock the counter before accessing to prevent updates from threads at the same time
  let mut visit_count = app_state.visit_count.lock().unwrap();
  let response = format!("{} {} times", health_check_response, visit_count);
  *visit_count += 1;
  HttpResponse::Ok().json(&response)
}

/// Handler for creating a new course.
///
/// # Arguments
///
/// * `new_course` - New course to create comming from the HTTP request.
/// * `app_state` - Container of the application state.
pub async fn new_course(
  new_course: web::Json<Course>,
  app_state: web::Data<AppState>,
) -> HttpResponse {
  println!("Received new course");
  // Lock the courses collection by the Mutex object.
  // Convert the collection into a consuming iterator .
  // Filter each course and look for courses only with the same tutor ID.
  // Get the corresponding number of existing, matching courses
  let course_count_for_user: usize = app_state
    .courses
    .lock()
    .unwrap()
    .clone()
    .into_iter()
    .filter(|course| course.tutor_id == new_course.tutor_id)
    .count();
  //.len();

  // Create a new course object with the given data. Generate the new course ID based on the number of filtered courses plus 1.
  let new_course = Course {
    tutor_id: new_course.tutor_id,
    course_id: Some(course_count_for_user + 1),
    course_name: new_course.course_name.clone(),
    posted_time: Some(Utc::now().naive_utc()),
  };
  // Add the new course objecto to the collection in the application state.
  app_state.courses.lock().unwrap().push(new_course);
  HttpResponse::Ok().json("Added course")
} // end fn new_course()

/// Gets the collection of courses belonging to the give tutor ID.
///
/// # Arguments
///
/// * `app_state` - Container of the application state.
/// * `params` - Collection of HTTP query parameters.
pub async fn get_courses_for_tutor(
  app_state: web::Data<AppState>,
  params: web::Path<usize>,
) -> HttpResponse {
  let tutor_id = params.into_inner();
  // Filter courses matching the tutor ID
  let filtered_courses = app_state
    .courses
    .lock()
    .unwrap()
    .clone()
    .into_iter()
    .filter(|course| course.tutor_id == tutor_id)
    .collect::<Vec<Course>>();

  if filtered_courses.len() > 0 {
    HttpResponse::Ok().json(filtered_courses)
  } else {
    HttpResponse::Ok().json("No courses found for tutor".to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::http::StatusCode;
  use std::sync::Mutex;

  #[actix_rt::test]
  async fn post_course_test() {
    let course = web::Json(Course {
      tutor_id: 1,
      course_name: "Hello, this is test course".into(),
      course_id: None,
      posted_time: None,
    });

    let app_state: web::Data<AppState> = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(1),
      courses: Mutex::new(vec![]),
    });

    let response: HttpResponse = new_course(course, app_state).await;
    assert_eq!(response.status(), StatusCode::OK);
  } // end fn post_course_test()

  #[actix_rt::test]
  async fn get_all_courses() {
    let app_state: web::Data<AppState> = web::Data::new(AppState {
      health_check_response: "".to_string(),
      visit_count: Mutex::new(0),
      courses: Mutex::new(vec![]),
    });

    let tutor_id: web::Path<usize> = web::Path::from((1));

    let response: HttpResponse = get_courses_for_tutor(app_state, tutor_id).await;

    assert_eq!(response.status(), StatusCode::OK);
  }
}
