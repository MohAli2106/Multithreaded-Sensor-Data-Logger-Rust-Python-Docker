use rand::Rng;
use chrono::{DateTime, Local};
use std::{
    fs::OpenOptions,
    io::Write,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(Debug)]
struct Sensor {
    id: u32,
    data: f64,
}

impl Sensor {
    fn new(id: u32) -> Self {
        Sensor { id, data: 0.0 }
    }

    fn collect_data(&mut self) {
        let mut rng = rand::thread_rng();
        self.data = rng.gen_range(20.0..30.0);
    }
}

fn time() -> String {
    let now = std::time::SystemTime::now();
    let datetime: DateTime<Local> = now.into();
    datetime.format("%H:%M:%S.%3f").to_string()
}

pub fn collect_sensor_data(sensor_id: u32) {
    let mut sensor = Sensor::new(sensor_id);
    println!("Sensor {} started", sensor_id);

    loop {
        sensor.collect_data();
        let timestamp = time();
        let log_line = format!(
            "Time: {}, Sensor: {}, Data: {:.2}",
            timestamp, sensor.id, sensor.data
        );
        println!("{}", log_line);

        thread::sleep(Duration::from_millis(500));
    }
}