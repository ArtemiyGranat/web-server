//! # Simple usage
//! ```no_run
//! use web_server::request::HttpRequest;
//! use web_server::response::HttpResponse;
//! use web_server::Server;
//! use web_server::method::HttpMethod;
//! use web_server::status_code::HttpStatusCode;
//! 
//! fn hello_world(_: HttpRequest) -> HttpResponse {
//!     HttpResponse::new(HttpStatusCode::OK).with_body("Hello, world!")
//! }
//!
//! fn main() {
//!     Server::new()
//!         .serve(HttpMethod::Get, "/", hello_world)
//!         .run("localhost", 8080)
//! }
//! ```
#[cfg(feature = "logger")]
use crate::logger::{LogLevel, Logger};
use crate::method::HttpMethod;
use crate::request::HttpRequest;
use crate::response::HttpResponse;
use crate::router::Router;
#[cfg(not(feature = "logger"))]
use crate::utils::DefaultLogger;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub mod file;
pub mod header;
pub mod http_version;
#[cfg(feature = "logger")]
mod logger;
pub mod method;
pub mod request;
pub mod response;
pub mod router;
pub mod status_code;
mod utils;

/// An HTTP server.
/// # Examples
/// ```no_run
/// use web_server::request::HttpRequest;
/// use web_server::response::HttpResponse;
/// use web_server::Server;
/// use web_server::method::HttpMethod;
/// use web_server::status_code::HttpStatusCode;
/// fn hello_world(_: HttpRequest) -> HttpResponse {
///     HttpResponse::new(HttpStatusCode::OK).with_body("Hello, world!")
/// }
///
/// fn main() {
///     Server::new()
///         .serve(HttpMethod::Get, "/", hello_world)
///         .run("localhost", 8080)
/// }
/// ```
pub struct Server {
    router: Router,
    #[cfg(feature = "logger")]
    logger: Logger,
    #[cfg(not(feature = "logger"))]
    logger: DefaultLogger,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Server {
    // TODO: `ignore` -> `no_run`
    /// Creates a new `Server`.
    /// # Example
    /// ```ignore
    /// Server::new()
    /// ```
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            #[cfg(feature = "logger")]
            logger: Logger::new(LogLevel::Debug).colored(),
            #[cfg(not(feature = "logger"))]
            logger: DefaultLogger::new(),
        }
    }

    /// Binds the server to address and port and starts listening to connections
    /// # Example
    /// ```ignore
    /// Server::new().run("localhost", 8080)
    /// ```
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

    /// Serves an incoming HTTP request and sends a response to it.
    /// Check the [`HttpMethod`](crate::method::HttpMethod) docs to see
    /// the available HTTP methods.
    pub fn serve<M>(
        mut self,
        method: M,
        target: &str,
        handler: fn(HttpRequest) -> HttpResponse,
    ) -> Self
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
                self.send_response(&addr, &mut stream, &request, &response);
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

    fn send_response(
        &self,
        _addr: &str,
        stream: &mut TcpStream,
        request: &HttpRequest,
        response: &HttpResponse,
    ) {
        if let Err(e) = stream.write_all(response.to_string(request.http_version()).as_bytes()) {
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
