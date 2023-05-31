#[cfg(test)]
mod tests {
    use web_server::{header::HttpHeader, request::HttpRequest};

    #[test]
    fn request_body_test() {
        let raw_request = "GET / HTTP/1.1\r\nHost: localhost\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, world!";
        let request = HttpRequest::new(raw_request).unwrap();
        assert_eq!(request.body(), "Hello, world!");
    }

    #[test]
    fn request_method_test() {
        let raw_request = "GET / HTTP/1.1\r\nHost: localhost\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, world!";
        let request = HttpRequest::new(raw_request).unwrap();
        assert_eq!(request.method().as_str(), "GET");
    }

    #[test]
    fn request_headers_test() {
        let raw_request = "GET / HTTP/1.1\r\nHost: localhost\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, world!";
        let request = HttpRequest::new(raw_request).unwrap();
        let headers = vec![
            HttpHeader::new("Host".to_string(), "localhost".to_string()),
            HttpHeader::new("Content-Type".to_string(), "text/plain".to_string()),
            HttpHeader::new("Content-Length".to_string(), "13".to_string()),
        ];
        assert_eq!(request.headers(), &headers);
    }

    #[test]
    fn request_target_test() {
        let raw_request = "GET / HTTP/1.1\r\nHost: localhost\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, world!";
        let request = HttpRequest::new(raw_request).unwrap();
        assert_eq!(request.target(), "/");
    }
}
