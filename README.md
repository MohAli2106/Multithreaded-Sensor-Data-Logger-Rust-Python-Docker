# Multithreaded Sensor Data Logger

This project is a decoupled sensor data logging system with separate Rust and Python components. The Rust component handles concurrent data collection and logging, while the Python component manages visualization. Both components can run independently or together, with Docker support for containerized deployment.

---

## Table of Contents

- [Features](#features)
- [Project Structure](#project-structure)
- [Components](#components)
- [Getting Started](#getting-started)
- [Docker Deployment](#docker-deployment)
- [CI/CD Integration](#cicd-integration)
- [Customization](#customization)
- [Troubleshooting](#troubleshooting)

---

## Features

- **Decoupled Architecture:** Independent Rust and Python components
- **Concurrent Data Collection:** Multi-threaded sensor simulation
- **Real-Time Logging:** Thread-safe data logging with timestamps
- **Data Visualization:** Configurable plotting system
- **Docker Support:** Containerized deployment options
- **CI/CD Integration:** Automated build and deployment pipelines

---

## Project Structure

```
Multithread_data_logger/
├── rust/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/
│       ├── main.rs
│       └── system.rs
├── python/
│   ├── plot_live.py
│   └── requirements.txt
├── docker/
│   ├── Dockerfile.rust
│   └── Dockerfile.python
├── .github/
│   └── workflows/
│       └── rust.yml
├── Jenkinsfile
└── README.md
```

---

## Components

### Rust Component
- Multi-threaded sensor simulation
- Thread-safe logging system
- Configurable sensor count and sampling rate

### Python Component
- Real-time data visualization
- Configurable plotting options
- Independent data processing

### Archticture
![system](https://github.com/MohAli2106/Multithreaded-Sensor-Data-Logger-Rust-Python-Docker/blob/master/system%20Arch.png)

---


## Getting Started

### Prerequisites
- Rust (edition 2021)
- Python 3.6+
- Docker (optional)
- Jenkins (optional)

### Running Components Separately

1. Start the Rust logger:
```sh
cd rust/
cargo run --release
```

2. Run the Python visualizer:
```sh
cd python/
python3 plot_live.py
```

---

## Docker Deployment

### Running Individual Components

1. Build and run Rust logger:
```sh
cd docker/
docker build -f Dockerfile.rust -t sensor-logger .
docker run --name logger sensor-logger
```

---

## CI/CD Integration

### Jenkins Pipeline

The Jenkinsfile provides:
- Separate build stages for Rust and Python
- Automated testing for both components
- Docker image building and pushing
- Deployment orchestration

---

## Customization

### Rust Component
- Modify sensor count in `main.rs`
- Adjust logging format in `system.rs`
- Configure sampling rates

### Python Component
- Customize plot styling
- Add new visualization types
- Modify data processing logic

### Docker Configuration
- Adjust resource limits
- Configure networking
- Modify build arguments

---

## Troubleshooting

### Rust Component
- Check log file permissions
- Verify thread limits
- Monitor resource usage

### Python Component
- Verify matplotlib installation
- Check log file access
- Monitor memory usage

### Docker Issues
- Check container logs
- Verify network connectivity
- Monitor resource constraints

### CI/CD Issues
- Review pipeline logs
- Check credentials
- Verify build dependencies
