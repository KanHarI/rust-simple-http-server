#![allow(dead_code)]

mod server;
use server::Server;

mod http;
use http::Method;

fn main() {
    let _get = Method::Get;

    let server = Server::new("0.0.0.0:8080".to_string());
    server.run();
}
