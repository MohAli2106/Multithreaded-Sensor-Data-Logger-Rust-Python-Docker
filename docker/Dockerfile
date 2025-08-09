#FROM rust:latest
#FROM python
#RUN sudo apt install python-pip
#RUN pip install matplotlib
#RUN cargo add libc

#WORKDIR /app

#COPY Cargo.toml Cargo.lock ./
#COPY src ./src
#COPY plot_sensor_data.py .

#CMD cargo run --release 
FROM rust:latest

# Install Python3 and pip
RUN apt-get update && \
    apt-get install -y python3-matplotlib && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Rust dependencies


WORKDIR /app

COPY rust/Cargo.toml rust/Cargo.lock ./
COPY src ./src
RUN cargo add libc

CMD ["cargo", "run", "--release"]
