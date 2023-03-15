// Uncomment this block to pass the first stage
use std::{net::TcpListener, io::{Write, Read}};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Connected!");
                let mut buf = [0; 512];
                loop {
                    let bytes_read = stream.read(&mut buf).unwrap();
                    if bytes_read == 0 {
                        println!("clent closed connection");
                        break;
                    }
                    stream.write(b"+PONG\r\n").unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
