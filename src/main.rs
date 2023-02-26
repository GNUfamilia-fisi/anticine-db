use std::collections::HashMap;
use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
};

const SERVER_ADDRESS: &str = "127.0.0.1:7878";

struct Database {
    data_map: HashMap<String, String>,
    filename: &'static str
}

impl Database {
    fn write(&self, line: &str) -> std::io::Result<()>  {
        std::fs::write(self.filename, line)?;
        Ok(())
    }

    fn set_key(&mut self, key: &str, value: &str) {
        println!("setting {} as {}", key, value);
        self.data_map.insert(key.to_string(), value.to_string());
        self.write(format!("{} {}", key, value).as_str()).unwrap();
    }

    fn get_key(&self, key: &str) -> Result<String, String> {
        println!("getting {}", key);
        match self.data_map.get(key) {
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
}

fn main() {
    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();
    println!("listening for connections at {}", SERVER_ADDRESS);
    // In-memory database
    let mut database = { Database {
        data_map: HashMap::new(),
        filename: "src/data.bin"
    } };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!\n");
                handle_connection(&stream, &mut database).unwrap();
            },
            Err(e) => {
                eprintln!("Failed to connect to socket {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: &TcpStream, db: &mut Database) -> std::io::Result<()> {
    stream.set_nonblocking(true).expect("Failed to set nonblocking stream");
    let mut still_connected = true;

    while still_connected {
        let mut stream_reader = BufReader::new(&mut stream);
        let mut result: Vec<u8> = Vec::new();
        let mut buff: [u8; 2048] = [0; 2048];

        loop {
            // Read the request
            match stream_reader.read(&mut buff) {
                Ok(n) if n > 0 => {
                    println!("read {} bytes", n);
                    result.extend_from_slice(&buff[..n]);
                },
                Ok(_) => {
                    println!("client disconnected");
                    still_connected = false;
                    break;
                },
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // println!("no data available");
                    break;
                },
                Err(_) => {
                    panic!("failed to read from stream");
                }
            }
        }
        if result.is_empty() {
            continue;
        }

        let contents = String::from_utf8(result).unwrap();
        println!("finished reaing contents: {}", contents.len());

        let mut iter = contents.split_ascii_whitespace();

        // Unpack request
        let method = iter.next().unwrap_or("");
        let key = iter.next().unwrap_or("");
        let value = iter.next().unwrap_or("");

        match method {
            "GET" => {
                let value = db.get_key(key).unwrap();
                let _ = stream.write(value.as_bytes()).unwrap();
            },
            "SET" => {
                db.set_key(key, value);
            }
            _ => {
                println!("invalid method");
            }
        }
    }

    Ok(())
}
