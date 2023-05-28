use crate::{header::Header, response::Response, file_handler::read_file};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    address: String,
    port: u16,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            address: "localhost".to_string(),
            port: 8080,
        }
    }
}

impl Server {
    pub fn new(address: &String, port: u16) -> Self {
        Self {
            address: address.to_string(),
            port,
        }
    }

    pub fn run(&self) {
        match TcpListener::bind((self.address.clone(), self.port)) {
            Ok(listener) => {
                println!("Server is listening at port {}", self.port);
                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => self.handle_connection(stream),
                        Err(e) => panic!("{}", e),
                    }
                }
            }
            Err(e) => {
                panic!("Could not bind a socket: {}", e);
            }
        }
    }
    
    // TODO: Add non-blocking I/O operations
    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream
            .read(&mut buffer)
            .expect("Could not read from stream");
        let response = Response::new(
            200,
            vec![Header::new(
                "Content-Type".to_string(),
                "text/html".to_string(),
            )],
            read_file("static/index.html").unwrap(),
        );
        stream
            .write_all(response.to_string().as_bytes())
            .expect("Could not write to stream");

        stream.flush().expect("Could not flush stream");
    }
}
