// rust/src/main.rs
extern crate lazy_static;  
#[macro_use]



mod system;

use std::thread;

fn main() {
    println!("Starting sensor data logger...");

    // Spawn two sensor threads
    let h1 = thread::spawn(|| system::collect_sensor_data(1));
    let h2 = thread::spawn(|| system::collect_sensor_data(2));

    // Keep main alive
    h1.join().expect("Sensor 1 thread panicked");
    h2.join().expect("Sensor 2 thread panicked");
}