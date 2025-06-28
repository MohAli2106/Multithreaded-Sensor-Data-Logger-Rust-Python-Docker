use rand::Rng;
use chrono::{DateTime, Local};
use std::{
    fs::OpenOptions,
    io::Write,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{Duration, SystemTime},
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
    let now = SystemTime::now();
    let datetime: DateTime<Local> = now.into();
    datetime.format("%H:%M:%S.%3f").to_string()
}

pub fn collect_sensor_data(
    sensor_id: u32,
    tx: mpsc::Sender<(u32, f64, String)>
) {
    let mut sensor = Sensor::new(sensor_id);
    loop {
        sensor.collect_data();
        let timestamp = time();
        //println!(
        //    "Time: {}, Sensor: {} Data: {:.2} C",
        //    timestamp, sensor.id, sensor.data
        //)
        
        tx.send((sensor.id, sensor.data, timestamp.clone()))
            .expect("Unable to send data");
        thread::sleep(Duration::from_secs(1));
    }
}


pub fn log_data(
    sensor_id: u32,
    data: f64,
    file_dir: &str,
    file_mutex: &Arc<Mutex<()>>,
    timestamp: String,
) {
    let _lock = file_mutex.lock().unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_dir)
        .expect("Unable to open file");

    writeln!(
        file,
        "Time: {}, Sensor: {}, Data: {:.2}",
        timestamp, sensor_id, data
    )
    .expect("Unable to write data to file");
}