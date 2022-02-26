use std::fs;

use yaml_rust::{yaml, Yaml, YamlLoader};

use crate::Topic;

pub struct FaCoseConfig {
    pub sensor: Sensors,
    pub astarte_device_params: AstarteDeviceParams,
}

pub struct AstarteDeviceParams {
    pub realm: String,
    pub device_id: String,
    pub token: String,
    pub pairing_url: String,
}

pub struct Sensors {
    pub files: Vec<File>,
}

pub struct File {
    name: String,
    path: String,
    interval: i64,
    interface_name: String,
    interface_endpoint: String
}

pub fn parse_config(config_file: &str) -> Result<FaCoseConfig, &'static str> {
    match fs::read_to_string(config_file) {
        Ok(contents) => {
            let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
            let doc = &docs[0];
            let astarte_params =
                parse_astarte_device_params(&doc["astarte_device"]).ok_or("Unable to parse astarte_device in config");
            let sensors = parse_sensors(&doc["sensors"]).ok_or("Unable to parse sensors in config");
            if astarte_params.is_ok() && sensors.is_ok() {
                Ok(FaCoseConfig {
                    astarte_device_params: astarte_params.unwrap(),
                    sensor: sensors.unwrap(),
                })
            } else {
                Err("Unable to parse config")
            }
        }
        Err(err) => {
            println!("{}", err);
            Err("Unable to parse config")
        }
    }
}

impl File {
    fn new(name: &str, path: &str, interval: i64, interface_name: &str, interface_endpoint: &str) -> File {
        File {
            name: name.clone().to_string(),
            path: path.clone().to_string(),
            interval,
            interface_name: interface_name.clone().to_string(),
            interface_endpoint: interface_endpoint.clone().to_string()
        }
    }

    pub fn get_abs_path(&self) -> String {
        let mut abs_path = self.path.clone();
        abs_path.push_str("/");
        abs_path.push_str(self.name.clone().as_str());
        abs_path
    }

    pub fn get_topic(&self) -> Topic {
        let mut abs_path = self.path.clone();
        abs_path.push_str("/");
        abs_path.push_str(self.name.clone().as_str());
        Topic { id: abs_path, name: self.interface_name.clone(), path: self.interface_endpoint.clone() }
    }

    pub fn get_interval(&self) -> u64 {
        self.interval as u64
    }
}

fn parse_astarte_device_params(doc: &yaml::Yaml) -> Option<AstarteDeviceParams> {
    match *doc {
        yaml::Yaml::Hash(ref v) => {
            let realm = v
                .get(&Yaml::String("realm".to_string()))
                .map(|realm| realm.as_str())
                .unwrap_or(None);

            let device_id = v
                .get(&Yaml::String("device_id".to_string()))
                .map(|device_id| device_id.as_str())
                .unwrap_or(None);

            let token = v
                .get(&Yaml::String("token".to_string()))
                .map(|token| token.as_str())
                .unwrap_or(None);

            let pairing_url = v
                .get(&Yaml::String("pairing_url".to_string()))
                .map(|pairing_url| pairing_url.as_str())
                .unwrap_or(None);

            if realm.is_some() && device_id.is_some() && token.is_some() && pairing_url.is_some() {
                Some(AstarteDeviceParams {
                    realm: realm.unwrap().to_string(),
                    device_id: device_id.unwrap().to_string(),
                    token: token.unwrap().to_string(),
                    pairing_url: pairing_url.unwrap().to_string(),
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_sensor_file(doc: &yaml::Yaml) -> Option<File> {
    match *doc {
        yaml::Yaml::Hash(ref v) => {
            let name = v
                .get(&Yaml::String("name".to_string()))
                .map(|name| name.as_str())
                .unwrap_or(None);

            let path = v
                .get(&Yaml::String("path".to_string()))
                .map(|name| name.as_str())
                .unwrap_or(None);

            let interval = v
                .get(&Yaml::String("interval".to_string()))
                .map(|interval| interval.as_i64())
                .unwrap_or(None);

            let interface_name = v
                .get(&Yaml::String("interface_name".to_string()))
                .map(|interface_name| interface_name.as_str())
                .unwrap_or(None);

            let interface_endpoint = v
                .get(&Yaml::String("interface_endpoint".to_string()))
                .map(|interface_endpoint| interface_endpoint.as_str())
                .unwrap_or(None);

            if name.is_some() && path.is_some() && interval.is_some()
                && interface_name.is_some() && interface_endpoint.is_some() {
                Some(File::new(name.unwrap(),
                               path.unwrap(),
                               interval.unwrap(),
                               interface_name.unwrap(),
                               interface_endpoint.unwrap())
                )
            } else {
                None
            }
        }
        _ => None,
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
