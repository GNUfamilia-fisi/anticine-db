use crate::database::{Database, DatabaseError};
use std::{
    net::TcpStream,
    io::{prelude::*, BufReader},
};

pub fn handle_connection(mut stream: &TcpStream, db: &mut Database) -> std::io::Result<()> {
    stream.set_nonblocking(true)?;
    let mut still_connected = true;

    while still_connected {
        let mut stream_reader = BufReader::new(&mut stream);
        let mut result: Vec<u8> = Vec::new();
        let mut buff: [u8; 2048] = [0; 2048];

        loop {
            // Read the request
            match stream_reader.read(&mut buff) {
                Ok(n) if n > 0 => {
                    // println!("read {} bytes", n);
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
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error reading from socket"));
                }
            }
        }
        if result.is_empty() {
            continue;
        }

        let contents = String::from_utf8_lossy(&result);
        println!("finished reaing contents: {}", contents.len());

        // This content can be merged with other contents of separate requests
        // so we need to split it into separate requests by '\r\n'

        let requests = contents.split("\r\n");

        for request in requests {
            if request.is_empty() {
                continue;
            }
            // print first 20 characters
            println!("request: {}", &request.chars().take(21).collect::<String>());
            // Unpack request
            let mut iter = request.split_ascii_whitespace();

            let method = iter.next().unwrap_or("");
            let key = iter.next().unwrap_or("");
            let value = iter.collect::<Vec<&str>>().join(" ");

            match method {
                "GET" => {
                    if let Ok(value) = db.get_key(key) {
                        stream.write(value.as_bytes()).unwrap();
                    }
                    else {
                        stream.write(b"not found").unwrap();
                    }
                },
                "SET" => {
                    // can throw KeyAlreadyExists and MalformedJson derived from DatabaseError
                    if let Err(e) = db.set_key(key, value) {
                        match e {
                            DatabaseError::KeyAlreadyExists => {
                                stream.write(b"already exists").unwrap();
                            },
                            DatabaseError::MalformedJson => {
                                stream.write(b"malformed json").unwrap();
                            },
                            _ => {
                                stream.write(b"unknown error").unwrap();
                            }
                        }
                    }
                    else {
                        stream.write(b"ok").unwrap();
                    }
                }
                _ => {
                    println!("invalid method");
                }
            };
        }
    }

    Ok(())
}
