use std::process::{Command, Output};

use crate::{Actuator, Publisher, Sensor, Topic};

#[derive(Clone)]
pub struct Terminal;

impl Terminal {
    fn run_command(&self, command: &str) -> Result<Output, &'static str> {
        let mut command_iter = command.split_whitespace();

        let command_name: String;

        match command_iter.next() {
            Some(val) => command_name = val.clone().to_string(),
            _ => {
                return Err("");
            }
        };

        let mut command = Command::new(command_name);
        for arg in command_iter {
            command.arg(arg);
        }
        Ok(command.output().expect("failed to execute process"))
    }
}

impl Actuator<String> for Terminal {
    fn write(&self, _: &str, value: &String) {
        let output = self.run_command(value.as_str());

        if output.is_err() {
            println!("Command executed with failing error code");
            return;
        }

        if output.unwrap().status.success() {
            println!("Command executed with failing error code");
            return;
        }
    }
}

impl Sensor<String> for Terminal {
    fn read(&self, id: &str) -> Result<String, &'static str> {
        let output = self.run_command(id)?;

        if !output.status.success() {
            return Err("Command executed with failing error code");
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

impl Publisher<String> for Terminal {
    fn publish(&self, topic: &Topic, value: String) {
        println!("publish path {} -> {}", topic.id, value)
    }
}

impl Publisher<i32> for Terminal {
    fn publish(&self, topic: &Topic, value: i32) {
        println!("publish path {} -> {}", topic.id, value)
    }
}
