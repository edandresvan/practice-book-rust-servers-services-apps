use std::{collections::HashMap, env, fs, path::Path};

use http::{
  http_request::{HttpRequest, Resource},
  http_response::HttpResponse,
};
use serde::{Deserialize, Serialize};

/// Represents a handler for HTTP requests.
pub trait Handler {
  /// Handles the given request to produce the respective response.
  ///
  /// # Arguments
  ///
  /// * `request`: HTTP request to handle.
  fn handle(request: &HttpRequest) -> HttpResponse;

  /// Loads the contents of the specified file from the server public directory.
  ///
  /// # Arguments
  ///
  /// * `filename`: Name of the file to load relative to the public directory.
  fn load_file(file_name: &str) -> Option<String> {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let full_path = format!("{}/{}", public_path, file_name);

    let contents: Result<String, std::io::Error> = fs::read_to_string(full_path);

    contents.ok()
  }
}

/// Represents the status of shipping order.
#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
  /// Unique identifier (ID) of the order.
  order_id: i32,
  /// Date of the order.
  order_date: String,
  /// Status of the order.
  order_status: String,
}

/// Represents a handler to serve the API (i.e. serve JSON files).
pub struct WebServiceHandler;

impl WebServiceHandler {
  /// Loads shipping orders from a JSON data file.
  fn load_json() -> Vec<OrderStatus> {
    let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
    let data_path = env::var("DATA_PATH").unwrap_or(default_path);

    let full_path = format!("{}/{}", data_path, "orders.json");

    let json_contents = fs::read_to_string(full_path).unwrap();

    let orders: Vec<OrderStatus> = serde_json::from_str(&json_contents.as_str()).unwrap();

    orders
  }
}

impl Handler for WebServiceHandler {
  fn handle(request: &HttpRequest) -> HttpResponse {
    let Resource::Path(p) = &request.resource;

    let route: Vec<&str> = p.split("/").collect();

    match route[2] {
      // Match the path '/api/shipping/orders'
      "shipping" if (route.len() > 2 && route[3] == "orders") => {
        let body = serde_json::to_string(&Self::load_json()).unwrap();
        let mut headers: HashMap<&str, &str> = HashMap::new();
        headers.insert("Content-Type", "application/json;charset=UTF-8");
        HttpResponse::new("200", Some(headers), Some(body))
      }
      _ => HttpResponse::new("404", None, Self::load_file("404.html")),
    }
  } // end fn handle()
}

/// Represents a handler to serve static web pages.
pub struct StaticPageHandler;

impl Handler for StaticPageHandler {
  fn handle(request: &HttpRequest) -> HttpResponse {
    // Obtain the path of the static page resource
    let Resource::Path(p) = &request.resource;
    let route: Vec<&str> = p.split("/").collect();

    match route[1] {
      // Serve the home page (index.html)
      "" => HttpResponse::new("200", None, Self::load_file("index.html")),
      // Serve the health page (health.html)
      "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
      // Serve any other page if the file exists
      path => match Self::load_file(path) {
        Some(contents) => {
          let mut headers: HashMap<&str, &str> = HashMap::new();

          // Set a header according to the file extension
          match Path::new(path).extension().unwrap().to_str() {
            Some("css") => headers.insert("Content-Type", "text/css"),
            Some("js") => headers.insert("Content-Type", "text/javascript"),
            None | _ => headers.insert("Content-Type", "text/html"),
          };

          HttpResponse::new("200", Some(headers), Some(contents))
        } // end some(contents) for an existing file
        // The requested page does not have a correspoding file, so respond with "Not Found"
        None => HttpResponse::new("404", None, Self::load_file("404.html")),
      },
    } // end match route[]
  } // end fn handle()
}

/// Represents a handler to serve "404 Not Found" pages.
pub struct PageNotFoundHandler;

impl Handler for PageNotFoundHandler {
  fn handle(_request: &HttpRequest) -> HttpResponse {
    HttpResponse::new("404", None, Self::load_file("404.html"))
  }
}
