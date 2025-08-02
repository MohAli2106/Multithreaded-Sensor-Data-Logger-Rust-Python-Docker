# Multithreaded Sensor Data Logger

This project is a multithreaded sensor data logger written in Rust. It simulates real-time data collection from multiple sensors, logs the data to a file, and provides live plotting using Python and Matplotlib. The project is designed for concurrency, reliability, and extensibility, and includes Docker and CI/CD integration for easy deployment.

---

## Table of Contents

- [Features](#features)
- [Project Structure](#project-structure)
- [How It Works](#how-it-works)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Build and Run Locally](#build-and-run-locally)
  - [Using Docker](#using-docker)
  - [CI/CD with Jenkins](#cicd-with-jenkins)
  - [GitHub Actions](#github-actions)
- [Customization](#customization)
- [File Descriptions](#file-descriptions)
- [Example Log Entry](#example-log-entry)
- [Troubleshooting](#troubleshooting)
- [License](#license)

---

## Features

- **Concurrent Data Collection:** Each sensor runs in its own thread, generating random temperature readings.
- **Real-Time Logging:** Sensor data is logged to `sensor_data.log` with timestamps in a thread-safe manner.
- **Live Plotting:** A Python script reads the log file and plots sensor data in real time.
- **Thread-Safe Synchronization:** Uses `Mutex` and `Arc` for safe shared access to resources.
- **Docker Support:** Easily build and run the system in a container.
- **CI/CD Integration:** Jenkins pipeline and GitHub Actions automate build, test, and deployment steps.

---

## Project Structure

```
Multithread_data_logger/
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── docfile
├── Jenkinsfile
├── plot_sensor_data.py
├── README.md
├── Rust.code-workspace
├── sensor_data.log
├── .github/
│   └── workflows/
│       └── rust.yml
├── src/
│   ├── main.rs
│   └── system.rs
└── target/
    └── ...
```

---

## How It Works

1. **Sensors:** Simulated sensors generate random temperature data every second in their own threads.
2. **Data Logging:** Each reading is written to `sensor_data.log` with a timestamp, using a mutex to prevent race conditions.
3. **Live Plotting:** The Python script [`plot_sensor_data.py`](plot_sensor_data.py) reads the log file and updates a live plot.
4. **Dockerized Deployment:** The system can be built and run in a Docker container for portability.
5. **CI/CD:** Jenkins and GitHub Actions automate building, testing, and running the project.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021)
- [Python 3](https://www.python.org/) with `matplotlib`
- (Optional) Docker, if you want to run in a container
- (Optional) Jenkins, for CI/CD

### Build and Run Locally

```sh
cargo run --release
```

This will start the logger, which writes data to `sensor_data.log` and launches the live plotting script.

### Using Docker

Build and run with Docker:

```sh
docker build -t data_logger .
docker run --name logger data_logger
```

The Dockerfile is named [`docfile`](docfile).

### CI/CD with Jenkins

The [Jenkinsfile](Jenkinsfile) automates:

- Cloning the repository
- Building the Docker image
- Stopping/removing any existing container
- Running the new container
- (Optionally) Archiving the sensor log
- Cleaning up Docker images and containers

### GitHub Actions

The workflow in [`.github/workflows/rust.yml`](.github/workflows/rust.yml) builds and tests the project on every push or pull request to the `master` branch.

---

## Customization

- **Add More Sensors:** Spawn more threads in [`src/main.rs`](src/main.rs).
- **Change Logging Frequency:** Adjust `thread::sleep` in [`src/system.rs`](src/system.rs).
- **Change Log Format:** Edit the `log_data` function in [`src/system.rs`](src/system.rs).
- **Plotting:** Modify [`plot_sensor_data.py`](plot_sensor_data.py) for different visualization needs.

---

## File Descriptions

- [`src/main.rs`](src/main.rs): Entry point. Starts sensor threads, logging, and the Python plotting script.
- [`src/system.rs`](src/system.rs): Contains sensor simulation and data logging logic.
- [`plot_sensor_data.py`](plot_sensor_data.py): Python script for live plotting of sensor data from the log file.
- [`sensor_data.log`](sensor_data.log): Output log file (created at runtime).
- [`Cargo.toml`](Cargo.toml): Rust project manifest and dependencies.
- [`docfile`](docfile): Dockerfile for building the container image.
- [`Jenkinsfile`](Jenkinsfile): Jenkins pipeline for CI/CD.
- [`.github/workflows/rust.yml`](.github/workflows/rust.yml): GitHub Actions workflow for CI.
- [`Rust.code-workspace`](Rust.code-workspace): VS Code workspace settings.
- [`.gitignore`](.gitignore): Git ignore rules.

---

## Example Log Entry

```
Time: 12:34:56.789, Sensor: 1, Data: 23.45
```

---

## Troubleshooting

- **Docker Issues:** Ensure Docker is running and you have permission to run containers.
- **Jenkins Pipeline Fails:** Check the console output for errors in build or run steps.
- **Plotting Issues:** Ensure Python 3 and `matplotlib` are installed.

---

## License

This project is for
