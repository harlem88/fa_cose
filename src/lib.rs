use std::time::Duration;

use tokio::{task, time};

pub mod astarte;
pub mod config;
pub mod file;
mod store;
pub mod terminal;

pub trait Sensor<T>: Sync {
    fn read(&self, id: &str) -> Result<T, &'static str>;
}

pub struct Topic {
    id: String,
    name: String,
    path: String,
}

pub trait Publisher<T>: Sync + Clone + Send + 'static {
    fn publish(&self, topic: &Topic, value: T);
}

pub trait Actuator<T> {
    fn write(&self, id: &str, value: &T);
}

pub fn schedule_telemetry<T>(
    topic: Topic,
    sensor: &'static impl Sensor<T>,
    publisher: impl Publisher<T>,
    period_seconds: u64,
) {
    let publisher = publisher.clone();
    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(period_seconds));
        loop {
            interval.tick().await;
            collect(&topic, sensor, &publisher).await;
        }
    });
}

pub async fn collect<T>(topic: &Topic, sensor: &impl Sensor<T>, publisher: &impl Publisher<T>) {
    let value: Result<T, &'static str> = sensor.read(&topic.id);

    match value {
        Ok(value) => {
            publisher.publish(&topic, value);
        }
        Err(err) => {
            println!("collect error {}", err)
        }
    }
}
