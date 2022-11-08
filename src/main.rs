#![allow(dead_code)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public_path: {}", public_path);
    // expected struct std::string::String (only "127.0.0.1:8080") found &str, try using '.to_string()'
    let server = Server::new("0.0.0.0:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/

/*
pub enum Option<T> {
    None,
    Some(T),
}
*/
    // let string = String::from("127.0.0.1:8080");
    // let string_slice = &string[10..]; -> = [10..14];, [0..3] = [..3]
    // let string_borrow: &str = &string; = 127.0.0.1:8080
    // let string_literal = "1234";
    // Conclusion -> string slice would be dangerous if using emoji(4 bytes)
    // for example "🪢🧤👑💍" if we slice [..5] it will be a part of second emoji, program will crash.

    // dbg!(string);
    // dbg!(string_slice);
    // Mac Apple Silicon not support dbg, rather use lldb
    // String consist of length, capacity, ptr
    // &str consist of length, ptr
