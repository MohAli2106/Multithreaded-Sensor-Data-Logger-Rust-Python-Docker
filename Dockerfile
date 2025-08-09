# Stage 1: Build Rust Binary
FROM rust:latest as builder

WORKDIR /app
COPY rust/Cargo.toml rust/Cargo.lock ./
COPY rust/src ./src
RUN cargo build --release --manifest-path=./Cargo.toml


# Stage 2: Runtime Image
FROM python:3.12-slim

# Install system deps
RUN apt-get update && \
    apt-get install -y \
        libpng-dev \
        libfreetype6-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

ENV MPLBACKEND=Agg

WORKDIR /workspace

# Install Python deps
COPY python/requirements.txt .
RUN pip install --no-cache-dir flask matplotlib


COPY --from=builder /app/target/release/sensor_monitor ./sensor_app


COPY python ./python


RUN mkdir -p ./static

EXPOSE 5000

# Run with stdout redirected to log file
CMD ["sh", "-c", "\
    echo ' Starting sensor logger...' && \
    ./sensor_app > sensor_data.log 2>&1 & \
    sleep 2 && \
    echo ' Starting Flask dashboard...' && \
    python3 ./python/app.py \
"]