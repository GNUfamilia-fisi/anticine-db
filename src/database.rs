use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader, SeekFrom};

#[derive(Debug)]
pub struct KeyNotFound;

#[derive(Debug)]
pub struct KeyAlreadyExists;

#[derive(Debug)]
pub struct MalformedJson;

pub enum DatabaseError {
    KeyNotFound,
    MalformedJson
}

impl From<KeyNotFound> for DatabaseError {
    fn from(_: KeyNotFound) -> Self {
        DatabaseError::KeyNotFound
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
    // - line must not contain line breaks
    // - line must be in the format: <key> <json>
    fn write(&self, line: &str) -> std::io::Result<()>  {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.filename)?;

        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
        Ok(())
    }

    // new line must be in the format: <key> <json> without line breaks
    fn overwrite(&mut self, key_to_find: &str, line_to_write: &str, parsed_json: &json::JsonValue) -> std::io::Result<bool> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.filename)?;

        let mut reader = BufReader::new(&file);

        let mut line = String::new();
        let mut n_bytes = 0;

        while let Ok(bytes) = reader.read_line(&mut line) {
            if bytes == 0 {
                break;
            }
            let key = line.split_ascii_whitespace().next().unwrap_or("");
            if key == key_to_find {
                // Replace line with dummy key: _, value = <_> to fill the line
                file.seek(SeekFrom::Start(n_bytes as u64))?;
                file.write_all(format!("_ {}\n", "_".repeat(bytes - 3)).as_bytes())?;

                file.seek(SeekFrom::End(0))?;
                file.write_all(line_to_write.as_bytes())?;
                file.write_all(b"\n")?;

                self.data_map.insert(key.to_string(), parsed_json.clone());

                return Ok(true);
            }
            n_bytes += bytes as u64;
            line.clear();
        }

        println!("key not found");
        Ok(false)
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

            if !key.is_empty() && !value.is_empty() && key != "_" {
                self.data_map.insert(key.to_string(), json::parse(value.as_str()).unwrap());
            }
        });
        Ok(())
    }

    pub fn set_key(&mut self, key: &str, value: String) -> Result<(), DatabaseError> {
        match json::parse(value.as_str()) {
            Ok(parsed_json) => {
                let to_write = format!("{} {}", key, parsed_json.dump());

                // if already exists, overwrite
                let key_exists = self.data_map.get(key).is_some();
                let mut overwritten = false;

                if key_exists {
                    overwritten = self.overwrite(key, &to_write, &parsed_json).unwrap_or(false);
                }

                if !key_exists || !overwritten {
                    self.data_map.insert(key.to_string(), parsed_json);
                    // save json to file
                    self.write(&to_write).unwrap_or_else(|e| {
                        eprintln!("Failed to write to file: {}", e);
                    });
                }
            },
            Err(e) => {
                println!("error parsing json: {}", e);
                return Err(MalformedJson.into());
            }
        };
        println!(" -> ok");
        Ok(())
    }

    pub fn get_key(&self, key: &str) -> Result<String, KeyNotFound> {
        // println!("getting {}", key);
        match self.data_map.get(key) {
            Some(value) => {
                println!(" -> ok");
                Ok(value.dump())
            },
            None => {
                println!(" -> not found");
                Err(KeyNotFound)
            }
        }
    }
}
