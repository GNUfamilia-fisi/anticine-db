use std::collections::HashMap;
use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
};

const SERVER_ADDRESS: &str = "127.0.0.1:7878";

fn main() {
    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();

    println!("listening for connections at {}", SERVER_ADDRESS);
    // In memory database
    let mut database: HashMap<String, String> = HashMap::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!\n");
                loop {
                    handle_connection(&stream, &mut database).unwrap();
                }
            },
            Err(e) => {
                eprintln!("Failed to connect to socket {}", e);
            }
        }
    }
}

fn set_key(key: &str, value: &str, db: &mut HashMap<String, String>) {
    println!("setting {} as {}", key, value);
    db.insert(key.to_string(), value.to_string());
}

fn get_key(key: &str, db: &HashMap<String, String>) -> Result<String, String> {
    println!("getting {}", key);
    match db.get(key) {
        Some(value) => {
            println!("value: {}", value);
            Ok(value.to_string())
        },
        None => {
            println!("key not found");
            Err("key not found".to_string())
        }
    }
}

fn handle_connection(mut stream: &TcpStream, db: &mut HashMap<String, String>) -> std::io::Result<()> {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut bytes: Vec<u8> = Vec::new();

    // Read the request
    buf_reader.read_until(b'\n', &mut bytes).unwrap();
    let request = String::from_utf8(bytes).unwrap();
    print!("request: {}", request);

    let mut iter = request.split_ascii_whitespace();

    // Unpack request
    let method = iter.next().unwrap_or("");
    let key = iter.next().unwrap_or("");
    let value = iter.next().unwrap_or("");

    match method {
        "GET" => {
            let value = get_key(key, db).unwrap();
            let _ = stream.write(value.as_bytes()).unwrap();
        },
        "SET" => {
            set_key(key, value, db);
        }
        _ => {
            println!("invalid method: {}", method);
        }
    }
    Ok(())
}
