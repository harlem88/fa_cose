pub mod config;
pub mod file;
pub mod terminal;

pub trait Sensor<T> {
    fn read(&self, id: &str) -> Result<T, &'static str>;
}

pub trait Publisher<T> {
    fn publish(&self, path: &str, value: T);
}

pub trait Actuator<T> {
    fn write(&self, id: &str, value: &T);
}

pub fn collect<T>(id: &str, sensor: impl Sensor<T>, publisher: impl Publisher<T>) {
    let value: Result<T, &'static str> = sensor.read(id);

    match value {
        Ok(value) => {
            publisher.publish(id, value);
        }
        Err(err) => {
            println!("collect error {}", err)
        }
    }
}
