mod handler;
mod router;
mod server;

use server::Server;

fn main() {
  // Start and then run the server
  let server: Server = Server::new("localhost:3000");
  server.run();
}
