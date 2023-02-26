use std::collections::HashMap;

pub struct Database {
    pub data_map: HashMap<String, String>,
    pub filename: &'static str
}

impl Database {
    fn write(&self, line: &str) -> std::io::Result<()>  {
        std::fs::write(self.filename, line)?;
        Ok(())
    }

    pub fn set_key(&mut self, key: &str, value: &str) {
        println!("setting {} as {}", key, value);
        self.data_map.insert(key.to_string(), value.to_string());
        self.write(format!("{} {}", key, value).as_str()).unwrap();
    }

    pub fn get_key(&self, key: &str) -> Result<String, String> {
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
