#![allow(non_snake_case)]
#[macro_use]

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

use serde::{Serialize, Deserialize};
use serde_json::{json};

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    userId: i32,
    friendsIds: Vec<i32>,
}


fn main() {

    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("\nServer listening on port 3333");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    println!("Connection established!");
                    handle_connection(stream)
                });
            }
            Err(e) => {
                println!("Connection failed. Error: {}", e);
            }
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        let response = "Hello from Server";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

}
