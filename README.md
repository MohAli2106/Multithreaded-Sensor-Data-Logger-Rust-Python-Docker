# Rust Multithreaded Sensor Data Logger

This project is a dynamic multithreaded sensor data logger built with Rust. It simulates real-time data collection from multiple sensors, showcasing Rust's powerful capabilities in concurrency and file I/O.

## Features

* Concurrent Data Collection: Each sensor operates on its own thread, mimicking real-world simultaneous data gathering.

* Real-Time Logging: Sensor readings are streamed to both the console and a TXT file instantly.

* Thread-Safe Synchronization: Employs Mutex to ensure smooth and conflict-free access to shared resources.

## How It Works

* Each thread simulates a unique sensor generating random temperature readings.

* The readings are displayed on the console and saved in real-time to a TXT file, now the data is sent into local server 127.0.0.1:7878.

* Synchronization through Mutex prevents data conflicts during file writing.

* pushing the data into server using some HTML implementation

  
## Customization

* Add More Sensors: Increase the number of threads in the main function to represent additional sensors.

* Modify Logging Frequency: Adjust the thread::sleep(Duration::from_secs(1)) interval for faster or slower logging.

* Change File Format: Enhance the log_data function to support formats like JSON or CSV.


## Key Concepts

* Multithreading: Utilizes std::thread for concurrent execution.

* File Handling: Writes sensor data to a TXT file with std::fs and std::io::Write.

* Thread Safety: Leverages std::sync::Mutex to coordinate shared file access.

* Timestamps: Uses chrono for precise time tracking of each data entry.






Unleash the power of Rust! 🌟
