# Multithreaded Sensor Data Logger in Rust

This project is a multithreaded sensor data logger written in Rust. It simulates real-time data collection from multiple sensors, logs the data to a file, and streams live updates to a web interface using a built-in TCP server.

## Features

- **Concurrent Data Collection:** Each sensor runs in its own thread, generating random temperature readings.
- **Real-Time Logging:** Sensor data is logged to `sensor_data.log` with timestamps.
- **Live Web Dashboard:** A simple HTML dashboard displays live sensor data using Server-Sent Events (SSE).
- **Thread-Safe Synchronization:** Uses `Mutex` and `Arc` for safe shared access to resources.
- **Custom TCP Server:** Serves both the dashboard and the SSE endpoint.

## How It Works

1. **Sensors:** Simulated sensors generate random temperature data every second.
2. **Data Logging:** Each reading is written to `sensor_data.log` with a timestamp.
3. **Web Dashboard:** Open [http://127.0.0.1:7877/](http://127.0.0.1:7877/) in your browser to view live updates.
4. **SSE Endpoint:** The dashboard connects to `/events` for real-time updates.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021)
- (Optional) Docker, if you want to run in a container

### Build and Run

```sh
cargo run --release
```

The server will start on `127.0.0.1:7877`. Open your browser to [http://127.0.0.1:7877/](http://127.0.0.1:7877/) to see live sensor data.

### Using Docker

Build and run with Docker:

```sh
docker build -t data_logger .
docker run -p 7877:7877 data_logger
```

## Project Structure

- `src/main.rs` — Entry point, starts sensors and server threads.
- `src/system.rs` — Sensor simulation and data logging logic.
- `src/tcp_server.rs` — TCP server for HTTP and SSE.
- `src/index.html` — Web dashboard.
- `sensor_data.log` — Output log file (created at runtime).

## Customization

- **Add More Sensors:** Spawn more threads in `main.rs`.
- **Change Logging Frequency:** Adjust `thread::sleep` in `system.rs`.
- **Change Log Format:** Edit the `log_data` function in `system.rs`.

## License

This project is for educational purposes.
