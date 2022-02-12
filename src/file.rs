use std::fs;
use crate::{Actuator, Sensor};

pub struct FileHandler;

impl Actuator<String> for FileHandler {
    fn write(&self, id: &str, value: &String) {
        let result = fs::write(id, value);

        if result.is_err() {
            println!("Unable to write file {}", result.err().unwrap());
        }
    }
}

impl Sensor<String> for FileHandler {
    fn read(&self, id: &str) -> Result<String, &'static str> {
        match fs::read_to_string(id) {
            Ok(contents) => {
                Ok(contents)
            }
            Err(error) => {
                println!("{}", error);
                Err("Unable to read file")
            }
        }
    }
}
