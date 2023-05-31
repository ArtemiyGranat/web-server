#[cfg(test)]
mod tests {
    use web_server::{request::HttpRequest, router::Router};
    // TODO: Write better tests
    // #[test]
    // fn response_code_test1() {
    //     let router = Router::new();
    //     let raw_request = "GET /index.html HTTP/1.1\r\nHost: localhost\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, world!";
    //     let request = Request::new(raw_request).unwrap();
    //     let response = router.handle_request(&request);
    //     assert_eq!(response.status_code().code(), 200);
    // }

    // #[test]
    // fn response_code_test2() {
    //     let router = Router::new();
    //     let raw_request = "GET / HTTP/1.1\r\nHost: localhost\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, world!";
    //     let request = Request::new(raw_request).unwrap();
    //     let response = router.handle_request(&request);
    //     assert_eq!(response.status_code().code(), 500);
    // }

    #[test]
    fn response_code_test3() {
        let router = Router::new();
        let raw_request = "GET /notfound.hmtl HTTP/1.1\r\nHost: localhost\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, world!";
        let request = HttpRequest::new(raw_request).unwrap();
        let response = router.handle_request(&request);
        assert_eq!(response.status_code().code(), 404);
    }
}