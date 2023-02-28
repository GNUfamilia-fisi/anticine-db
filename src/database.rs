use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct KeyNotFound;

pub struct Database {
    pub data_map: HashMap<String, json::JsonValue>,
    pub filename: &'static str
}

impl Database {
    fn write(&self, line: &str) -> std::io::Result<()>  {
        let mut file = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(self.filename)?;

        file.write_all(line.as_bytes())?;

        Ok(())
    }

    pub fn load(&mut self) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(self.filename)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        contents.split('\n').for_each(|line| {
            let mut iter = line.split_ascii_whitespace();
            let key = iter.next().unwrap_or("");
            let value = iter.next().unwrap_or("");

            if !key.is_empty() && !value.is_empty() {
                self.data_map.insert(key.to_string(), json::parse(value).unwrap());
            }
            else {
                // bad line, ignore
            }
        });
        Ok(())
    }

    // TODO: this function should return a Result
    pub fn set_key(&mut self, key: &str, value: &str) {
        match json::parse(value) {
            Ok(parsed) => {
                println!("setting {} as {}", key, value);
                let mut value = parsed.dump();
                value.push('\n');
                let to_write = format!("{} {}", key, value);

                // save json to memory
                self.data_map.insert(key.to_string(), parsed);
                // save json to file
                self.write(&to_write).unwrap_or_else(|e| {
                    eprintln!("Failed to write to file: {}", e);
                });
            },
            Err(e) => {
                println!("error parsing json: {}", e);
            }
        }
    }

    pub fn get_key(&self, key: &str) -> Result<String, KeyNotFound> {
        println!("getting {}", key);
        match self.data_map.get(key) {
            Some(value) => {
                println!("value: {}", value);
                Ok(value.dump())
            },
            None => {
                println!("key not found");
                Err(KeyNotFound)
            }
        }
    }
}
