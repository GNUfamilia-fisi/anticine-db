use crate::database::Database;
use std::{
    net::TcpStream,
    io::{prelude::*, BufReader},
};

pub fn handle_connection(mut stream: &TcpStream, db: &mut Database) -> std::io::Result<()> {
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