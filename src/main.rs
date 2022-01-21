use std::io::prelude::*;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        println!("connection established");

        let mut buf: [u8; 512] = [0; 512];
        while let Ok(bytes_read) = stream.read(&mut buf) {
            if bytes_read <= 0 {
                break;
            }
            print!("{}", String::from_utf8_lossy(&buf[..]));
        }
    }
}
