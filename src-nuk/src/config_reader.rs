use serde_json::Value;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

pub struct ConfigReader {
    config_path: String,
    example_config_path: String,
}

impl ConfigReader {
    pub fn new(config_path: String, example_config_path: String) -> Self {
        ConfigReader {
            config_path,
            example_config_path,
        }
    }

    pub fn read_config(&self) -> Value {
        if let Ok(mut file) = File::open(&self.config_path) {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read config file");
            let json: Value = serde_json::from_str(&contents).expect("Failed to parse config file");
            return json;
        } else {
            self.create_config_from_example();
            return self.read_config();
        }
    }

    fn create_config_from_example(&self) {
        if let Ok(mut example_file) = File::open(&self.example_config_path) {
            let mut example_contents = String::new();
            example_file
                .read_to_string(&mut example_contents)
                .expect("Failed to read example config file");

            let mut config_file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&self.config_path)
                .expect("Failed to create config file");

            config_file
                .write_all(example_contents.as_bytes())
                .expect("Failed to write config file");

            println!("Created config file from example");
        } else {
            println!("Example config file not found");
        }
    }
}
