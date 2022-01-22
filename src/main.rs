use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use colored::*;

fn print_info(msg: &str) {
    println!("{}", msg.blue());
}

fn handle_connection(mut stream: TcpStream) {
    print_info("connection established");
    let mut buf: [u8; 512] = [0; 512];
    while let Ok(bytes_read) = stream.read(&mut buf) {
        if bytes_read <= 0 {
            break;
        }
        print!("{}", String::from_utf8_lossy(&buf[..]));
        buf = [0; 512]; // zero out buffer
    }
    println!();
    print_info("connection closed");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
