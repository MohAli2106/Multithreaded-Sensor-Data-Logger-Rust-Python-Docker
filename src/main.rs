use std::sync::{Arc, mpsc};
use std::thread;
use std::process::Command;

mod system;
use system::{collect_sensor_data, log_data};

fn main() {
    // Start the Python plotting script first and keep the handle
    let mut plot_child = Command::new("python3")
        .arg("plot_sensor_data.py")
        .spawn()
        .expect("Failed to run Python plot script");

    // Start the logging threads as before
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

    // When logging is done, terminate the plot process
    #[cfg(unix)]
    {
        // Add libc to your dependencies: cargo add libc
        unsafe { libc::kill(plot_child.id() as i32, libc::SIGTERM); }
    }
    #[cfg(windows)]
    {
        let _ = plot_child.kill();
    }
    let _ = plot_child.wait();
}