use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub struct Tcpserv {
    address: String,
    data_sto: Arc<Mutex<Vec<String>>>, 
}

impl Tcpserv {
    pub fn new(address: &str) -> Self {
        Tcpserv {
            address: address.to_string(),
            data_sto: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn handle_serv(mut stream: TcpStream, data_sto: Arc<Mutex<Vec<String>>>) {
        let mut reader = BufReader::new(&stream);
        let mut request = String::new();
    
        if reader.read_line(&mut request).is_ok() {
            if request.starts_with("GET /events") {
               
                let response_headers = "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nConnection: keep-alive\r\n\r\n";
                stream
                    .write_all(response_headers.as_bytes())
                    .expect("Could not write headers");
    
                // Send all historical data
                let historical_data = {
                    let data = data_sto.lock().unwrap();
                    data.clone() // Clone the data to avoid holding the lock for too long
                };
    
                for data in historical_data {
                    let event = format!("data: {}\n\n", data);
                    stream
                        .write_all(event.as_bytes())
                        .expect("Could not write historical data");
                }
    
                // Keep the connection open and send updates
                loop {
                    let new_data = {
                        let data = data_sto.lock().unwrap();
                        data.last().cloned() // Get the latest data
                    };
    
                    if let Some(data) = new_data {
                        let event = format!("data: {}\n\n", data);
                        stream
                            .write_all(event.as_bytes())
                            .expect("Could not write event");
                    }
    
                    thread::sleep(Duration::from_secs(1)); // Simulate real-time updates
                }
            } else if request.starts_with("GET / ") {
                // Serve the index.html file
                let response_headers = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n";
                let response_body = include_str!("index.html"); // Include the HTML file
    
                let response = format!("{}{}", response_headers, response_body);
                stream
                    .write_all(response.as_bytes())
                    .expect("Could not write to stream");
            }
        }
    }
    pub fn server_on(&self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;
        println!("Server is ready on {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let data_sto = self.data_sto.clone();
                    thread::spawn(move || Tcpserv::handle_serv(stream, data_sto));
                    println!("New connection established");
                }
                Err(e) => eprintln!("Could not connect: {}", e),
            }
        }

        Ok(())
    }

    pub fn update_data(&self, new_data: String) {
        let mut data = self.data_sto.lock().unwrap();
        data.push(new_data); // Append new data to the storage
    }
}