// Uncomment this block to pass the first stage
use std::{net::{TcpListener}, io::Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                _stream.write(b"+PONG\r\n");
                _stream.flush();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
