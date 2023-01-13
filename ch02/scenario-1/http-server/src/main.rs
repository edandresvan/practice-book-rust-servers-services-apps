use std::net::TcpListener;

fn main() {
  let socket_address: &str = "127.0.0.1:3000";
  let connection_listener : TcpListener= TcpListener::bind(&socket_address).unwrap();
  println!("Running TCP Server on {}", &socket_address);

  for stream in connection_listener.incoming() {
    let _stream = stream.unwrap();
    println!("connection established");
  }
}
