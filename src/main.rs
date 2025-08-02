use std::sync::{Arc, mpsc};
use std::thread;

mod system;
use system::{collect_sensor_data, log_data};

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    let h1 = thread::spawn(move || collect_sensor_data(1, tx1));
    let h2 = thread::spawn(move || collect_sensor_data(2, tx2));

    let file_dir = "sensor_data.log";
    let file_mutex = Arc::new(std::sync::Mutex::new(()));
    
    for (sensor_id, data, timestamp) in rx {
        log_data(sensor_id, data, file_dir, &file_mutex, timestamp);
    }

    h1.join().expect("Thread 1 panicked");
    h2.join().expect("Thread 2 panicked");
}