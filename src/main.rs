use std::sync::{Arc, mpsc};
use std::thread;

mod system;
use system::{collect_sensor_data, log_data};

mod tcp_server;
use tcp_server::Tcpserv;

fn main() {
    let address = "127.0.0.1:7878".to_string();
    let server = Arc::new(Tcpserv::new(&address));

    // Start the server in a separate thread
    let server_clone = server.clone();
    let server_thread = thread::spawn(move || {
        if let Err(e) = server_clone.server_on() {
            eprintln!("Could not start server: {}", e);
        }
    });

    let (tx, rx) = mpsc::channel();

    
    let tx1 = tx.clone();
    let tx2 = tx.clone();


    let h1 = thread::spawn({
        let server = server.clone();
        move || collect_sensor_data(1, tx1, server)
    });

    let h2 = thread::spawn({
        let server = server.clone();
        move || collect_sensor_data(2, tx2, server)
    });

  
    let file_dir = "sensor_data.log";
    let file_mutex = Arc::new(std::sync::Mutex::new(()));
    for (sensor_id, data, timestamp) in rx {
        log_data(sensor_id, data, file_dir, &file_mutex, timestamp);
    }

    h1.join().expect("Thread 1 panicked");
    h2.join().expect("Thread 2 panicked");
    server_thread.join().expect("Server thread panicked");
}