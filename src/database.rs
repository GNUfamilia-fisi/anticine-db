use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct KeyNotFound;

#[derive(Debug)]
pub struct KeyAlreadyExists;

#[derive(Debug)]
pub struct MalformedJson;

pub enum DatabaseError {
    KeyNotFound,
    KeyAlreadyExists,
    MalformedJson
}

impl From<KeyNotFound> for DatabaseError {
    fn from(_: KeyNotFound) -> Self {
        DatabaseError::KeyNotFound
    }
}

impl From<KeyAlreadyExists> for DatabaseError {
    fn from(_: KeyAlreadyExists) -> Self {
        DatabaseError::KeyAlreadyExists
    }
}

impl From<MalformedJson> for DatabaseError {
    fn from(_: MalformedJson) -> Self {
        DatabaseError::MalformedJson
    }
}

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
            let value = iter.collect::<Vec<&str>>().join(" ");

            if !key.is_empty() && !value.is_empty() {
                self.data_map.insert(key.to_string(), json::parse(value.as_str()).unwrap());
            }
            else {
                // bad line, ignore
            }
        });
        Ok(())
    }

    pub fn set_key(&mut self, key: &str, value: String) -> Result<(), DatabaseError> {
        if !self.data_map.get(key).is_none() {
            return Err(KeyAlreadyExists.into());
        }
        match json::parse(value.as_str()) {
            Ok(parsed) => {
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
                return Err(MalformedJson.into());
            }
        }
        Ok(())
    }

    pub fn get_key(&self, key: &str) -> Result<String, KeyNotFound> {
        println!("getting {}", key);
        match self.data_map.get(key) {
            Some(value) => {
                println!(" -> OK");
                Ok(value.dump())
            },
            None => {
                println!(" -> NOT FOUND");
                Err(KeyNotFound)
            }
        }
    }
}
