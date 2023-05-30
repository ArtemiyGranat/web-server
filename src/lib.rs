use method::Method;
use response::Response;

#[cfg(feature = "logger")]
use crate::logger::{LogLevel, Logger};
#[cfg(not(feature = "logger"))]
use crate::utils::DefaultLogger;
use crate::{request::Request, router::Router};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

// mod file_handler;
pub mod header;
#[cfg(feature = "logger")]
pub mod logger;
mod method;
pub mod request;
pub mod response;
pub mod router;
mod status_code;
mod utils;

pub struct Server {
    router: Router,
    #[cfg(feature = "logger")]
    logger: Logger,
    #[cfg(not(feature = "logger"))]
    logger: DefaultLogger,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            router: Router::new(),
            #[cfg(feature = "logger")]
            logger: Logger::new(LogLevel::Debug).colored(),
            #[cfg(not(feature = "logger"))]
            logger: DefaultLogger::new(),
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            #[cfg(feature = "logger")]
            logger: Logger::new(LogLevel::Debug).colored(),
            #[cfg(not(feature = "logger"))]
            logger: DefaultLogger::new(),
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
                        Err(e) => {
                            self.logger
                                .error(format!("Could not accept connection: {}", e));
                        }
                    }
                }
            }
            Err(e) => {
                self.logger.error(format!("Could not bind a socket: {}", e));
            }
        }
    }

    pub fn serve<M>(&mut self, method: M, path: &str, handler: fn(Request) -> Response)
    where
        M: Into<Method>,
    {
        self.router.get(path, handler);
    }

    pub fn get(mut self, path: &str, handler: fn(Request) -> Response) -> Self {
        self.router.get(path, handler);
        self
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
        #[cfg(feature = "logger")]
        self.logger.request_received(&addr, &request);
        let response = self.router.handle_request(&request);
        if let Err(e) = stream.write_all(response.to_string().as_bytes()) {
            self.logger
                .error(format!("Could not write to stream: {}", e));
            return;
        }
        #[cfg(feature = "logger")]
        self.logger.request_completed(&addr, &response);
        if let Err(e) = stream.flush() {
            self.logger.error(format!("Could not flush stream: {}", e));
        }
    }
}
