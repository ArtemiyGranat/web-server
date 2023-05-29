#[cfg(feature = "logger")]
use crate::logger::{LogLevel, Logger};
use crate::{request::Request, router::Router};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    router: Router,
    #[cfg(feature = "logger")]
    logger: Logger,
}

impl Server {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            #[cfg(feature = "logger")]
            logger: Logger::new(LogLevel::Debug).colored(),
        }
    }

    pub fn run(&self, address: &str, port: u16) {
        match TcpListener::bind((address, port)) {
            Ok(listener) => {
                #[cfg(feature = "logger")]
                self.logger
                    .info(format!("Server is listening at port {}", port));
                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => self.handle_connection(stream),
                        Err(e) =>
                        {
                            #[cfg(feature = "logger")]
                            self.logger
                                .error(format!("Could not accept connection: {}", e));
                            #[cfg(not(feature = "logger"))]
                            eprintln!("Could not accept connection: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                #[cfg(feature = "logger")]
                self.logger.error(format!("Could not bind a socket: {}", e));
                #[cfg(not(feature = "logger"))]
                eprintln!("Could not bind a socket: {}", e);
            }
        }
    }

    // TODO: Add non-blocking I/O operations
    fn handle_connection(&self, mut stream: TcpStream) {
        let addr = match stream.peer_addr() {
            Ok(addr) => addr.ip().to_string(),
            Err(e) => {
                #[cfg(feature = "logger")]
                self.logger
                    .error(format! {"Failed to detect IP address: {}", e});
                #[cfg(not(feature = "logger"))]
                eprintln!("Failed to detect IP address: {}", e);
                return;
            }
        };

        let mut buffer = [0; 1024];
        if let Err(e) = stream.read(&mut buffer) {
            #[cfg(feature = "logger")]
            self.logger
                .error(format!("Could not read from stream: {}", e));
            #[cfg(not(feature = "logger"))]
            eprintln!("Could not read from stream: {}", e);
            return;
        }
        let raw_request = String::from_utf8_lossy(&buffer);
        let request = Request::new(&raw_request);
        if let Err(e) = request {
            #[cfg(feature = "logger")]
            self.logger
                .error(format!("Could not parse a request from {}: {}", addr, e));
            #[cfg(not(feature = "logger"))]
            eprintln!("Could not parse a request from {}: {}", addr, e);
            return;
        }
        let request = request.unwrap();
        #[cfg(feature = "logger")]
        self.logger.request_received(&addr, &request);
        let response = self.router.handle_request(&request);
        if let Err(e) = stream.write_all(response.to_string().as_bytes()) {
            #[cfg(feature = "logger")]
            self.logger
                .error(format!("Could not write to stream: {}", e));
            #[cfg(not(feature = "logger"))]
            eprintln!("Could not write to stream: {}", e);
            return;
        }
        #[cfg(feature = "logger")]
        self.logger.request_completed(&addr, &response);
        if let Err(e) = stream.flush() {
            #[cfg(feature = "logger")]
            self.logger.error(format!("Could not flush stream: {}", e));
            #[cfg(not(feature = "logger"))]
            eprintln!("Could not flush stream: {}", e);
        }
    }
}
