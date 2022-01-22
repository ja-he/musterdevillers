use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

use colored::*;

fn print_warning(msg: &str) {
    println!("{}", msg.red());
}

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

#[allow(dead_code)]
struct HttpGetRequest {
    request_uri: String,
    http_version: String,
    headers: Vec<String>,
}

#[allow(dead_code)]
enum HttpRequest {
    Get(HttpGetRequest),
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

fn read_request_from(stream: &TcpStream) -> Option<HttpRequest> {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut line = String::new();
    return match reader.read_line(&mut line) {
        Ok(0) => None,
        Ok(_) => Some({
            let tokens: Vec<&str> = line.split_whitespace().collect();
            match tokens[0] {
                "GET" => {
                    assert!(tokens.len() == 3);
                    let request_uri = tokens[1].to_string();
                    let http_version = tokens[2].to_string();
                    let headers: Vec<String> = Vec::new();
                    HttpRequest::Get(HttpGetRequest {
                        request_uri,
                        http_version,
                        headers,
                    })
                }
                _ => panic!("unhandlable request type"),
            }
        }),
        _ => None,
    };
}

fn handle_connection(stream: TcpStream) {
    print_info("connection established");

    let request = read_request_from(&stream);
    match request {
        Some(HttpRequest::Get(_get_request)) => {
            print_info("received get request");
            send_default_response(stream);
            print_info("connection closed");
        }
        Some(_) => print_warning("unhandlable request type"),
        None => print_warning("no request"),
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
