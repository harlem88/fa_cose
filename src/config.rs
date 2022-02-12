use std::any::Any;
use std::fs;

use yaml_rust::{yaml, Yaml, YamlLoader};

pub struct Sensors {
    pub files: Vec<File>,
}

pub struct File {
    name: String,
    path: String,
}

pub fn parse_config(config_file: &str) -> Result<Sensors, &'static str> {
    match fs::read_to_string(config_file) {
        Ok(contents) => {
            let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
            let doc = &docs[0];
            parse_sensors(&doc["sensors"]).ok_or("Unable to parse config")
        }
        Err(err) => {
            println!("{}", err);
            Err("Unable tp parse config")
        }
    }
}

impl File {
    fn new(name: &str, path: &str) -> File {
        File { name: name.clone().to_string(), path: path.clone().to_string() }
    }

    pub fn get_abs_path(&self) -> String{
        let mut abs_path = self.path.clone();
        abs_path.push_str("/");
        abs_path.push_str(self.name.clone().as_str());
        abs_path
    }
}

fn parse_sensor_file(doc: &yaml::Yaml) -> Option<File> {
    match *doc {
        yaml::Yaml::Hash(ref v) => {
            let name = v.get(&Yaml::String("name".to_string())).map(
                |name| name.as_str()
            ).unwrap_or(None);

            let path = v.get(&Yaml::String("path".to_string())).map(
                |name| name.as_str()
            ).unwrap_or(None);

            if name.is_some() && path.is_some() {
                Some(File::new(name.unwrap(), path.unwrap()))
            } else {
                None
            }
        }
        _ => { None }
    }
}

fn parse_sensors(doc: &yaml::Yaml) -> Option<Sensors> {
    let mut sensors = Sensors { files: Vec::new() };

    match *doc {
        yaml::Yaml::Hash(ref h) => {
            for (key, keys) in h {
                match key.as_str() {
                    Some(sensor_type) if sensor_type == "file" => {
                        parse_sensor_file(keys).map(|file| {
                            sensors.files.push(file);
                        });
                    }
                    _ => {
                        println!("Unsupported sensor type")
                    }
                }
            }
        }
        _ => {
            println!("Unsupported type")
        }
    }
    Some(sensors)
}

