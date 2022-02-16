use std::thread::JoinHandle;
use std::time::Duration;

use tokio::{task, time};

pub mod config;
pub mod file;
pub mod terminal;

pub trait Sensor<T>: Sync {
    fn read(&self, id: &str) -> Result<T, &'static str>;
}

pub trait Publisher<T>: Sync {
    fn publish(&self, path: &str, value: T);
}

pub trait Actuator<T> {
    fn write(&self, id: &str, value: &T);
}

pub async fn collect<T>(id: &str, sensor: &impl Sensor<T>, publisher: &impl Publisher<T>) {
    let value: Result<T, &'static str> = sensor.read(&id);

    match value {
        Ok(value) => {
            publisher.publish(&id, value);
        }
        Err(err) => {
            println!("collect error {}", err)
        }
    }
}

pub fn schedule_telemetry<T>(
    id: String,
    sensor: &'static impl Sensor<T>,
    publisher: &'static impl Publisher<T>,
    period_seconds: u64,
) {
    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(period_seconds));
        loop {
            interval.tick().await;
            collect(&id, sensor, publisher).await;
        }
    });
}
