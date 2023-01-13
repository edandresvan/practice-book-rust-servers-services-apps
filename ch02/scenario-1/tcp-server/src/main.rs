use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
  // Create the TCP Server
  let socket_address: &str = "127.0.0.1:3000";
  let connection_listener: TcpListener = TcpListener::bind(&socket_address).unwrap();
  println!("Running TCP Server on {}", &socket_address);

  // Wait for TCP client connections
  for stream in connection_listener.incoming() {
    // Stream of bytes comming from the client
    let mut stream: TcpStream = stream.unwrap();
    println!("Connection established with a client.");

    // Read data from the stream comming from the client
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // Send data to the stream and to the client
    stream.write(&mut buffer).unwrap();
  }
}
