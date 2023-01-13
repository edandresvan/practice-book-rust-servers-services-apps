use std::collections::HashMap;

/// Represents an HTTP method.
#[derive(Debug, PartialEq)]
pub enum Method {
  /// The HTTP GET method.
  GET,
  /// The HTTP POST method.
  POST,
  /// Unknown HTTP method.
  UNINITIALIZED,
}

impl From<&str> for Method {
  fn from(value: &str) -> Self {
    match value {
      "GET" => Method::GET,
      "POST" => Method::POST,
      _ => Method::UNINITIALIZED,
    }
  }
}

/// Represents the version of the HTTP protocol.
#[derive(Debug, PartialEq)]
pub enum Version {
  /// HTTP/1.1 version.
  V1_1,
  /// HTTP/2.0 version.
  V2_0,
  /// Unknown HTTP version.
  UNINITIALIZED,
}

impl From<&str> for Version {
  fn from(value: &str) -> Self {
    match value {
      "HTTP/1.1" => Version::V1_1,
      _ => Version::UNINITIALIZED,
    }
  }
}

/// Represents the path to a REST resource.
#[derive(Debug, PartialEq)]
pub enum Resource {
  Path(String),
}

/// Represents an HTTP request.
#[derive(Debug, PartialEq)]
pub struct HttpRequest {
  /// HTTP method of the request.
  pub method: Method,
  /// HTTP version of the request.
  pub version: Version,
  /// Path of the requested REST resource.
  pub resource: Resource,
  /// Set of headers of the HTTP request.
  pub headers: HashMap<String, String>,
  /// Body message of the request.
  pub message_body: String,
}

impl From<String> for HttpRequest {
  fn from(req: String) -> Self {
    let mut parsed_method: Method = Method::UNINITIALIZED;
    let mut parsed_version: Version = Version::UNINITIALIZED;
    let mut parsed_resource: Resource = Resource::Path("".to_string());
    let mut parsed_headers: HashMap<String, String> = HashMap::new();
    let mut parsed_message_body: String = "".to_string();

    // Process each line of the incoming HTTP request
    for line in req.lines() {
      // Process the request line
      if line.contains("HTTP") {
        let (method, resource, version) = process_req_line(line);
        parsed_method = method;
        parsed_version = version;
        parsed_resource = resource;
      } else if line.contains(":") {
        let (key, value) = process_header_line(line);
        parsed_headers.insert(key, value);
      } else if line.len() == 0 {
        // It is an emmpty line followed by the body
      } else {
        parsed_message_body = line.to_string();
      }
    }

    // Create the request object
    HttpRequest {
      method: parsed_method,
      version: parsed_version,
      resource: parsed_resource,
      headers: parsed_headers,
      message_body: parsed_message_body.to_string(),
    }
  }
}

/// Returns the method, resource and version components of the given request line.
///
/// # Arguments
///
/// * `line`: HTTP request line.
///  
fn process_req_line(line: &str) -> (Method, Resource, Version) {
  // Split the request line by whitespaces into a iterator
  let mut words: std::str::SplitWhitespace = line.split_whitespace();

  // Extract the HTTP method
  let method: &str = words.next().unwrap();

  // Extract the path of the resource
  let resource_path: String = words.next().unwrap().to_string();

  // Extract the HTTP version
  let version: &str = words.next().unwrap();

  (method.into(), Resource::Path(resource_path), version.into())
}

/// Returns the set of HTTP headers from the given string line.
///
/// # Arguments
///
/// * `line`: String line containing the HTTP headers.
fn process_header_line(line: &str) -> (String, String) {
  let mut key = "".to_string();
  let mut value = "".to_string();

  // Split the key and the value parts only by the first colon (:)
  let header_items: Vec<&str> = line.splitn(2, ":").collect::<Vec<&str>>();
  // Asign the key and the value only if both parts are present
  if header_items.len() == 2 {
    key = header_items[0].trim().to_string();
    value = header_items[1].trim().to_string();
  }

  (key, value)
}
// ----------------------------------------------------------- //

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_method_into() {
    let method_get: Method = "GET".into();
    assert_eq!(method_get, Method::GET);

    let method_post: Method = "POST".into();
    assert_eq!(method_post, Method::POST);

    let method_uninitialized: Method = "".into();
    assert_eq!(method_uninitialized, Method::UNINITIALIZED);

    let method_uninitialized: Method = "Other".into();
    assert_eq!(method_uninitialized, Method::UNINITIALIZED);
  }

  #[test]
  fn test_version_into() {
    let version_1: Version = "HTTP/1.1".into();
    assert_eq!(version_1, version_1);

    let version_2: Version = "HTTP/2.0".into();
    assert_eq!(version_2, Version::UNINITIALIZED);

    let uninitialized: Version = "".into();
    assert_eq!(uninitialized, Version::UNINITIALIZED);

    let uninitialized: Version = "HTTP".into();
    assert_eq!(uninitialized, Version::UNINITIALIZED);

    let uninitialized: Version = "Other".into();
    assert_eq!(uninitialized, Version::UNINITIALIZED);
  }

  #[test]
  fn test_read_http() {
    let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\nHello World!\r\n");

    let req: HttpRequest = s.into();

    assert_eq!(req.method, Method::GET);
    assert_eq!(req.resource, Resource::Path("/greeting".to_string()));
    assert_eq!(req.version, Version::V1_1);

    let mut headers_expected: HashMap<String, String> = HashMap::new();
    headers_expected.insert("Host".into(), "localhost:3000".into());
    headers_expected.insert("User-Agent".into(), "curl/7.81.0".into());
    headers_expected.insert("Accept".into(), "*/*".into());

    assert_eq!(req.headers, headers_expected);

    assert_eq!("Hello World!".to_string(), req.message_body);
  }
}
