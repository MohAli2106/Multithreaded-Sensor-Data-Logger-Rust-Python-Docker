# Multithreaded Sensor Data Logger in Rust

This project is a multithreaded sensor data logger written in Rust. It simulates real-time data collection from multiple sensors and logs the data to a file. The system is designed for concurrency, reliability, and extensibility, and includes Docker and CI/CD integration for easy deployment.

---

## Features

- **Concurrent Data Collection:** Each sensor runs in its own thread, generating random temperature readings.
- **Real-Time Logging:** Sensor data is logged to `sensor_data.log` with timestamps in a thread-safe manner.
- **Thread-Safe Synchronization:** Uses `Mutex` and `Arc` for safe shared access to resources.
- **Docker Support:** Easily build and run the system in a container.
- **CI/CD Pipeline:** Jenkins pipeline automates build, test, and deployment steps.

---

## How It Works

1. **Sensors:** Simulated sensors generate random temperature data every second in their own threads.
2. **Data Logging:** Each reading is written to `sensor_data.log` with a timestamp, using a mutex to prevent race conditions.
3. **Dockerized Deployment:** The system can be built and run in a Docker container for portability.
4. **CI/CD:** Jenkinsfile automates cloning, building, running, and cleaning up the Docker environment.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021)
- (Optional) Docker, if you want to run in a container
- (Optional) Jenkins, for CI/CD

### Build and Run Locally

```sh
cargo run --release
```

The logger will start and write data to `sensor_data.log`.

### Using Docker

Build and run with Docker:

```sh
docker build -t data_logger .
docker run --name logger data_logger
```

### Using Jenkins (CI/CD)

The [Jenkinsfile](Jenkinsfile) automates the following steps:
- Clones the repository
- Builds the Docker image
- Stops and removes any existing container
- Runs the new container
- Archives the sensor log (customize as needed)
- Cleans up Docker images and containers

---

## Project Structure

- [`src/main.rs`](src/main.rs) — Entry point, starts sensors and manages logging.
- [`src/system.rs`](src/system.rs) — Sensor simulation and data logging logic.
- [`sensor_data.log`](sensor_data.log) — Output log file (created at runtime).
- [`Jenkinsfile`](Jenkinsfile) — Jenkins pipeline for CI/CD.
- [`docfile`](docfile) — Dockerfile for building the container image.

---

## Customization

- **Add More Sensors:** Spawn more threads in [`main.rs`](src/main.rs).
- **Change Logging Frequency:** Adjust `thread::sleep` in [`system.rs`](src/system.rs).
- **Change Log Format:** Edit the `log_data` function in [`system.rs`](src/system.rs).

---

## Example Log Entry

```
Time: 2024-06-01 12:34:56, Sensor: 1, Data: 23.45
```

---

## Troubleshooting

- **Docker Issues:** Ensure Docker is running and you have permission to run containers.
- **Jenkins Pipeline Fails:** Check the console output for errors in build or run steps.

---

## License

This project is
