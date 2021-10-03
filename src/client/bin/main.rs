#[macro_use]
use std::io::Write;
use std::net::TcpStream;
use std::{io, thread};

use serde::{Serialize, Deserialize};
use serde_json::{json};

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    userId: i32,
    friendsIds: Vec<i32>,
}

fn main() {
    match TcpStream::connect("0.0.0.0:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            println!("Enter userId: ");
            let mut userIdStr = String::new();
            io::stdin().read_line(&mut userIdStr).expect("error: unable to read userIdStr");
            let userId: i32 = userIdStr.trim().parse().unwrap();

            println!("Enter friends IDs(comma separated): ");
            let mut friendsIdsStr = String::new();
            io::stdin().read_line(&mut friendsIdsStr).expect("error: unable to read friendsIdsStr");

            let friendsIds: Vec<i32> = friendsIdsStr.trim().split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            let payload = Payload { userId, friendsIds };
            println!("payloadJson = {}", json!(payload));
            // println!("payloadJsonBytes = {:?}", payloadJson.as_bytes());

            return;


            /*let msg = b"Hello!";
            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            readStream(stream);*/
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
