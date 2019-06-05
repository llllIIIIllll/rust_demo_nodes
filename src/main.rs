#![deny(deprecated)]
extern crate tokio;

use tokio::net::TcpStream;

fn main() {
    // Parse the address of whatever server we're talking to
    let addr = "127.0.0.1:6142".parse().unwrap();
    let client = TcpStream::connect(&addr);

    // Following snippets come here...
    // test for ci
}