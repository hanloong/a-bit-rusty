extern crate mio;

use mio::*;
use mio::tcp::{TcpListener, TcpStream};

struct WebSocketServer;

impl Handler for WebSocketServer {
  type Timeout = usize;
  type Message = ();
}

fn main() {
  let addr = "127.0.0.1:1000".parse().unwrap();
  let server = TcpListener::bind(&addr).unwrap();

  let mut event_loop = EventLoop::new().unwrap();

  event_loop.register(&server, Token(0));

  let sock = TcpStream::connect(&addr).unwrap();

  // Create a new instance of the handler struct
  let mut handler = WebSocketServer;

  event_loop.run(&mut handler).unwrap();
}
