use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use colored::*;

fn print_info(msg: &str) {
    println!("{}", msg.blue());
}

fn send_default_response(mut stream: TcpStream) {
    let default_content = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Musterdevillers</title>
  </head>
  <body>
    <h1>Hello</h1>
    <p>Hi from Musterdevillers!</p>
  </body>
</html>"#;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        default_content.len(),
        default_content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    print_info("connection established");
    let mut buf: [u8; 4096] = [0; 4096];
    stream.read(&mut buf).unwrap();
    print!("{}", String::from_utf8_lossy(&buf[..]));
    println!();

    print_info("responding...");

    send_default_response(stream);

    print_info("connection closed");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
