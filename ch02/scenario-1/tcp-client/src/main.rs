use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
  // Create the TCP client
  let server_address: &str = "127.0.0.1:3000";
  let mut stream: TcpStream = TcpStream::connect(&server_address).unwrap();

  // Send a message to the server
  stream.write("Hello you server".as_bytes()).unwrap();

  // Read the response from the server
  let mut buffer: [u8; 20] = [0; 20];
  let bytes_count: usize = stream.read(&mut buffer).unwrap();

  println!("{}", bytes_count);

  println!(
    "Received a response from the server: {:?}",
    str::from_utf8(&buffer).unwrap().trim()
  );
}
