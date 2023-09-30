#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use http::Method;
use http::Request;
use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod http;
mod server;
mod website_handler;
fn main() {
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBIC_PATH").unwrap_or(default_path);
    println!("public path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
