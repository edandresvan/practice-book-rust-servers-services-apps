use std::{net::{TcpListener, TcpStream}, io::Read};

use http::http_request::HttpRequest;

use crate::router::Router;

/// Represents a server.
pub struct Server<'a> {
  /// Socket address to listen connections.
  socket_address: &'a str,
}

impl<'a> Server<'a> {
  /// Creates a new [`Server`] object.
  /// 
  /// # Argument
  /// 
  /// * `socket_address`: Socket address to listen new connections.
  pub fn new(socket_address: &'a str) -> Self {
    Self {
      socket_address: socket_address,
    }
  }

  /// Runs the server
  pub fn run(&self) {
    // Start the server on the socket address
    let connection_listener : TcpListener= TcpListener::bind(self.socket_address).unwrap();

    println!("Server running on {}", self.socket_address);

    // Listen and waits for new connections
    for stream in connection_listener.incoming() {
      let mut stream : TcpStream = stream.unwrap();
      println!("Connection established with client.");
      // Create the request from the byte stream received
      let mut read_buffer = [0; 90];
      stream.read(&mut read_buffer).unwrap();

      let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();

      // Route the request to the appropiate handler
      Router::route(req, &mut stream);
    }
  }
}
