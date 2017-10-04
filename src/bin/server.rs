extern crate grpctest;

use grpctest::server::server;

fn main() {
    server::start();
    println!("Hello, world!");
}
