#[cfg(feature = "logger")]
use crate::logger::{LogLevel, Logger};
#[cfg(not(feature = "logger"))]
use crate::utils::DefaultLogger;

use crate::{method::HttpMethod, request::HttpRequest, response::HttpResponse, router::Router};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub mod file;
pub mod header;
#[cfg(feature = "logger")]
pub mod logger;
pub mod method;
pub mod request;
pub mod response;
pub mod router;
pub mod status_code;
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
                    .info(&format!("Server is listening at port {}", port));

                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => self.handle_connection(stream),
                        Err(e) => {
                            self.logger
                                .error(&format!("Could not accept connection: {}", e));
                        }
                    }
                }
            }
            Err(e) => {
                self.logger
                    .error(&format!("Could not bind a socket: {}", e));
            }
        }
    }

    pub fn serve<M>(
        &mut self,
        method: M,
        target: &str,
        handler: fn(HttpRequest) -> HttpResponse,
    ) -> &mut Self
    where
        M: Into<HttpMethod>,
    {
        match method.into() {
            HttpMethod::Unknown => {
                self.logger.error("Invalid HTTP method");
            }
            method => {
                self.router.add_route(method, target, handler);
            }
        }
        self
    }

    // TODO: Add non-blocking I/O operations
    fn handle_connection(&self, mut stream: TcpStream) {
        let addr = match stream.peer_addr() {
            Ok(addr) => addr.ip().to_string(),
            Err(e) => {
                self.logger
                    .error(&format! {"Failed to detect IP address: {}", e});
                return;
            }
        };

        let raw_request = match self.read_request(&mut stream) {
            Ok(raw_request) => raw_request,
            Err(e) => {
                self.logger
                    .error(&format!("Could not read request from {}: {}", addr, e));
                return;
            }
        };

        match HttpRequest::new(&raw_request) {
            Ok(request) => {
                #[cfg(feature = "logger")]
                self.logger.request_received(&addr, &request);

                let response = self.router.handle_request(&request);
                self.send_response(&addr, &mut stream, &response);
            }
            Err(e) => self
                .logger
                .error(&format!("Could not parse a request from {}: {}", addr, e)),
        }
    }

    fn read_request(&self, stream: &mut TcpStream) -> Result<String, std::io::Error> {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;
        let raw_request = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        Ok(raw_request)
    }

    fn send_response(&self, _addr: &str, stream: &mut TcpStream, response: &HttpResponse) {
        if let Err(e) = stream.write_all(response.to_string().as_bytes()) {
            self.logger
                .error(&format!("Could not write to stream: {}", e));
            return;
        }
        #[cfg(feature = "logger")]
        self.logger.request_completed(&_addr, &response);
        if let Err(e) = stream.flush() {
            self.logger.error(&format!("Could not flush stream: {}", e));
        }
    }
}
