mod database;
mod connection;
use database::Database;
use connection::handle_connection;

use std::collections::HashMap;
use std::{
    net::TcpListener,
};

const SERVER_ADDRESS: &str = "127.0.0.1:7878";

fn main() {
    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap_or_else(|e| {
        panic!("Failed to bind to socket {}: {}", SERVER_ADDRESS, e);
    });

    println!("listening for connections at {}", SERVER_ADDRESS);

    let mut database = { Database {
        data_map: HashMap::new(),
        filename: "src/data.bin"
    } };

    for stream in listener.incoming() {
        // get socket address
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr().unwrap();
                println!("Connection from {}", addr);
                handle_connection(&stream, &mut database).unwrap();
            },
            Err(e) => {
                eprintln!("Failed to connect to socket {}", e);
            }
        }
    }
}
