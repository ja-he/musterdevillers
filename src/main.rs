use std::io::prelude::*;
use std::io::BufReader;
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

fn get_linewise_til_crlfcrlf(stream: &TcpStream) -> Vec<String> {
    let mut lines = Vec::new();
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        line = line.trim().to_string();
        if line.is_empty() {
            break;
        }
        lines.push(line.clone());
        line = String::from("")
    }

    return lines;
}

fn handle_connection(stream: TcpStream) {
    print_info("connection established");

    for line in get_linewise_til_crlfcrlf(&stream).iter() {
        println!("{}", line);
    }

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
