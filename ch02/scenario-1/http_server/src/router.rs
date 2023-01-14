use http::{
  http_request::{HttpRequest, Method, Resource},
  http_response::HttpResponse,
};

use std::io::prelude::*;

use crate::handler::{
  Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler,
};

/// Represents a router to process requests.
pub struct Router;

impl Router {
  pub fn route(
    request: HttpRequest,
    stream: &mut impl Write,
  ) -> () {
    match request.method {
      // Process GET requests
      Method::GET => {
        // Route according to the resource requested
        match &request.resource {
          Resource::Path(p) => {
            let route: Vec<&str> = p.split("/").collect();

            match route[1] {
              // Process a request to the API (/api)
              "api" => {
                // Invoke the web service
                let response: HttpResponse = WebServiceHandler::handle(&request);
                let _ = response.send_response(stream);
              }
              // Process a requet to the page handler (/**)
              _ => {
                let response: HttpResponse = StaticPageHandler::handle(&request);
                let _ = response.send_response(stream);
              }
            }
          }
        }
      } // end match GET
      // Any other method is regarded as not found
      _ => {
        let response: HttpResponse = PageNotFoundHandler::handle(&request);
        let _ = response.send_response(stream);
      }
    }
  } // end fn route()
}
