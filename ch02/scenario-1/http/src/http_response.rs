use std::collections::HashMap;
use std::io::{Result, Write};

/// Represents an HTTP response to a request.
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
  /// HTTP protocol version.
  version: &'a str,
  /// HTTP status numerical code.
  status_code: &'a str,
  // HTTP status text.
  status_text: &'a str,
  /// Headers of the HTTP response.
  headers: Option<HashMap<&'a str, &'a str>>,
  /// Body of the HTTP response.
  body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
  fn default() -> Self {
    Self {
      version: "HTTP/1.1",
      status_code: "200",
      status_text: "OK",
      headers: None,
      body: None,
    }
  }
}

impl<'a> From<HttpResponse<'a>> for String {
  fn from(value: HttpResponse<'a>) -> String {
    let res = value.clone();
    format!(
      "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
      &res.version(),
      &res.status_code(),
      &res.status_text(),
      &res.headers(),
      if res.body.is_some() {
        res.body().len()
      } else {
        0
      },
      &res.body()
    )
  }
}

impl<'a> HttpResponse<'a> {
  /// Creates an new [`HttpResponse`] object with default values and the given parameters.
  ///
  /// # Arguments
  ///
  /// * `status_code`: HTTP status numerical code for the response.
  /// * `headers`: Set of HTTP headers for the response.
  /// * `body`: Contents of the HTTP body for the response.
  pub fn new(
    status_code: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
  ) -> HttpResponse<'a> {
    let mut response: HttpResponse<'a> = HttpResponse::default();

    if status_code != "200" {
      response.status_code = status_code.into();
    }

    response.headers = match &headers {
      Some(_h) => headers,
      None => {
        let mut h: HashMap<&str, &str> = HashMap::new();
        h.insert("Content-Type", "text/html");
        Some(h)
      }
    };

    response.status_text = match response.status_code {
      "200" => "OK",
      "400" => "Bad Request",
      "404" => "Not Found",
      "500" => "Internal Server Error",
      _ => "Not Found",
    };

    response.body = body;

    response
  } // end fn new()

  /// Gets the HTTP version.
  fn version(&self) -> &str {
    self.version
  }

  /// Gets the HTTP status numerical code.
  fn status_code(&self) -> &str {
    self.status_code
  }

  /// Gets the HTTP status text.
  fn status_text(&self) -> &str {
    self.status_text
  }

  /// Gets the HTTP headers as a single text string.
  fn headers(&self) -> String {
    let mut header_string: String = "".to_string();

    for (k, v) in self.headers.as_ref().unwrap() {
      header_string = format!("{}{}:{}\r\n", header_string, k, v);
    }
    header_string
  }

  /// Gets the HTTP body.
  pub fn body(&self) -> &str {
    match &self.body {
      Some(b) => b.as_str(),
      None => "",
    }
  }

  /// Sends this response as a byte stream.
  ///
  /// # Arguments
  ///
  /// * `write_stream`: Byte stream writer. Recommended: a TCP stream
  pub fn send_response(
    &self,
    write_stream: &mut impl Write,
  ) -> Result<()> {
    let response = self.clone();
    let _ = write!(write_stream, "{}", String::from(response));
    Ok(())
  } // end fn send_response()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_response_struct_creation_200() {
    let response_actual = HttpResponse::new(
      "200",
      None,
      Some("Item was shipped on 21st Dec 2020".to_string()),
    );

    let response_expected = HttpResponse {
      version: "HTTP/1.1",
      status_code: "200",
      status_text: "OK",
      headers: {
        let mut h: HashMap<&str, &str> = HashMap::new();
        h.insert("Content-Type", "text/html");
        Some(h)
      },
      body: Some("Item was shipped on 21st Dec 2020".to_string()),
    };

    assert_eq!(response_actual, response_expected);
  }

  #[test]
  fn test_response_struct_creation_400() {
    let response_actual = HttpResponse::new(
      "404",
      None,
      Some("Item was shipped on 21st Dec 2020".to_string()),
    );

    let response_expected = HttpResponse {
      version: "HTTP/1.1",
      status_code: "404",
      status_text: "Not Found",
      headers: {
        let mut h: HashMap<&str, &str> = HashMap::new();
        h.insert("Content-Type", "text/html");
        Some(h)
      },
      body: Some("Item was shipped on 21st Dec 2020".to_string()),
    };

    assert_eq!(response_actual, response_expected);
  }

  #[test]
  fn test_http_response_creation() {
    let response_actual = HttpResponse {
      version: "HTTP/1.1",
      status_code: "404",
      status_text: "Not Found",
      headers: {
        let mut h: HashMap<&str, &str> = HashMap::new();
        h.insert("Content-Type", "text/html");
        Some(h)
      },
      body: Some("Item was shipped on 21st Dec 2020".to_string()),
    };

    let http_actual: String = response_actual.into();
    let http_expected = "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 33\r\n\r\nItem was shipped on 21st Dec 2020";

    assert_eq!(http_actual, http_expected);
  }

  #[test]
  fn test_http_response_empty_body() {
    let response_actual = HttpResponse {
      version: "HTTP/1.1",
      status_code: "404",
      status_text: "Not Found",
      headers: {
        let mut h: HashMap<&str, &str> = HashMap::new();
        h.insert("Content-Type", "text/html");
        Some(h)
      },
      body: None,
    };

    let http_actual: String = response_actual.into();
    let http_expected =
      "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 0\r\n\r\n";

    assert_eq!(http_actual, http_expected);
  }
}
