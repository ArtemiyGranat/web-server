use crate::{
    logger::{LogLevel, Logger},
    request::Request,
    router::Router,
};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    router: Router,
    logger: Logger,
}

impl Server {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            logger: Logger::new(LogLevel::Debug).colored(),
        }
    }

    pub fn run(&self, address: &str, port: u16) {
        match TcpListener::bind((address, port)) {
            Ok(listener) => {
                self.logger
                    .info(format!("Server is listening at port {}", port));
                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => self.handle_connection(stream),
                        Err(e) => self
                            .logger
                            .error(format!("Could not accept connection: {}", e)),
                    }
                }
            }
            Err(e) => {
                self.logger.error(format!("Could not bind a socket: {}", e));
            }
        }
    }

    // TODO: Add non-blocking I/O operations
    fn handle_connection(&self, mut stream: TcpStream) {
        let addr = match stream.peer_addr() {
            Ok(addr) => addr.ip().to_string(),
            Err(e) => {
                self.logger
                    .error(format! {"Failed to detect IP address: {}", e});
                return;
            }
        };

        let mut buffer = [0; 1024];
        if let Err(e) = stream.read(&mut buffer) {
            self.logger
                .error(format!("Could not read from stream: {}", e));
            return;
        }
        let raw_request = String::from_utf8_lossy(&buffer);
        let request = Request::new(&raw_request);
        if let Err(e) = request {
            self.logger
                .error(format!("Could not parse a request from {}: {}", addr, e));
            return;
        }
        let request = request.unwrap();

        self.logger.request_received(&addr, &request);
        let response = self.router.handle_request(&request);
        if let Err(e) = stream.write_all(response.to_string().as_bytes()) {
            self.logger
                .error(format!("Could not write to stream: {}", e));
            return;
        }

        self.logger.request_completed(&addr, &response);
        if let Err(e) = stream.flush() {
            self.logger.error(format!("Could not flush stream: {}", e));
        }
    }
}
