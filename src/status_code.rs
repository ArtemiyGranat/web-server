use std::fmt;

/// An HTTP response's status code.
#[derive(Clone, Copy)]
pub struct HttpStatusCode(u16);

// TODO: Implement unknown code const/struct/etc
impl HttpStatusCode {
    pub const CONTINUE: HttpStatusCode = HttpStatusCode(100);
    pub const SWITCHING_PROTOCOLS: HttpStatusCode = HttpStatusCode(101);
    pub const PROCESSING: HttpStatusCode = HttpStatusCode(102);
    pub const EARLY_HINTS: HttpStatusCode = HttpStatusCode(103);

    pub const OK: HttpStatusCode = HttpStatusCode(200);
    pub const CREATED: HttpStatusCode = HttpStatusCode(201);
    pub const ACCEPTED: HttpStatusCode = HttpStatusCode(202);
    pub const NON_AUTHORITATIVE_INFORMATION: HttpStatusCode = HttpStatusCode(203);
    pub const NO_CONTENT: HttpStatusCode = HttpStatusCode(204);
    pub const RESET_CONTENT: HttpStatusCode = HttpStatusCode(205);
    pub const PARTIAL_CONTENT: HttpStatusCode = HttpStatusCode(206);
    pub const MULTI_STATUS: HttpStatusCode = HttpStatusCode(207);
    pub const ALREADY_REPORTED: HttpStatusCode = HttpStatusCode(208);
    pub const IM_USED: HttpStatusCode = HttpStatusCode(226);

    pub const MULTIPLE_CHOICES: HttpStatusCode = HttpStatusCode(300);
    pub const MOVED_PERMANENTLY: HttpStatusCode = HttpStatusCode(301);
    pub const FOUND: HttpStatusCode = HttpStatusCode(302);
    pub const SEE_OTHER: HttpStatusCode = HttpStatusCode(303);
    pub const NOT_MODIFIED: HttpStatusCode = HttpStatusCode(304);
    pub const USE_PROXY: HttpStatusCode = HttpStatusCode(305);
    pub const TEMPORARY_REDIRECT: HttpStatusCode = HttpStatusCode(307);
    pub const PERMANENT_REDIRECT: HttpStatusCode = HttpStatusCode(308);

    pub const BAD_REQUEST: HttpStatusCode = HttpStatusCode(400);
    pub const UNAUTHORIZED: HttpStatusCode = HttpStatusCode(401);
    pub const PAYMENT_REQUIRED: HttpStatusCode = HttpStatusCode(402);
    pub const FORBIDDEN: HttpStatusCode = HttpStatusCode(403);
    pub const NOT_FOUND: HttpStatusCode = HttpStatusCode(404);
    pub const METHOD_NOT_ALLOWED: HttpStatusCode = HttpStatusCode(405);
    pub const NOT_ACCEPTABLE: HttpStatusCode = HttpStatusCode(406);
    pub const PROXY_AUTHENTICATION_REQUIRED: HttpStatusCode = HttpStatusCode(407);
    pub const REQUEST_TIMEOUT: HttpStatusCode = HttpStatusCode(408);
    pub const CONFLICT: HttpStatusCode = HttpStatusCode(409);
    pub const GONE: HttpStatusCode = HttpStatusCode(410);
    pub const LENGTH_REQUIRED: HttpStatusCode = HttpStatusCode(411);
    pub const PRECONDITION_FAILED: HttpStatusCode = HttpStatusCode(412);
    pub const PAYLOAD_TOO_LARGE: HttpStatusCode = HttpStatusCode(413);
    pub const URI_TOO_LONG: HttpStatusCode = HttpStatusCode(414);
    pub const UNSUPPORTED_MEDIA_TYPE: HttpStatusCode = HttpStatusCode(415);
    pub const RANGE_NOT_SATISFIABLE: HttpStatusCode = HttpStatusCode(416);
    pub const EXPECTATION_FAILED: HttpStatusCode = HttpStatusCode(417);
    pub const IM_A_TEAPOT: HttpStatusCode = HttpStatusCode(418);
    pub const MISDIRECTED_REQUEST: HttpStatusCode = HttpStatusCode(421);
    pub const UNPROCESSABLE_ENTITY: HttpStatusCode = HttpStatusCode(422);
    pub const LOCKED: HttpStatusCode = HttpStatusCode(423);
    pub const FAILED_DEPENDENCY: HttpStatusCode = HttpStatusCode(424);
    pub const TOO_EARLY: HttpStatusCode = HttpStatusCode(425);
    pub const UPGRADE_REQUIRED: HttpStatusCode = HttpStatusCode(426);
    pub const PRECONDITION_REQUIRED: HttpStatusCode = HttpStatusCode(428);
    pub const TOO_MANY_REQUESTS: HttpStatusCode = HttpStatusCode(429);
    pub const REQUEST_HEADER_FIELDS_TOO_LARGE: HttpStatusCode = HttpStatusCode(431);
    pub const UNAVAILABLE_FOR_LEGAL_REASONS: HttpStatusCode = HttpStatusCode(451);

    pub const INTERNAL_SERVER_ERROR: HttpStatusCode = HttpStatusCode(500);
    pub const NOT_IMPLEMENTED: HttpStatusCode = HttpStatusCode(501);
    pub const BAD_GATEWAY: HttpStatusCode = HttpStatusCode(502);
    pub const SERVICE_UNAVAILABLE: HttpStatusCode = HttpStatusCode(503);
    pub const GATEWAY_TIMEOUT: HttpStatusCode = HttpStatusCode(504);
    pub const HTTP_VERSION_NOT_SUPPORTED: HttpStatusCode = HttpStatusCode(505);
    pub const VARIANT_ALSO_NEGOTIATES: HttpStatusCode = HttpStatusCode(506);
    pub const INSUFFICIENT_STORAGE: HttpStatusCode = HttpStatusCode(507);
    pub const LOOP_DETECTED: HttpStatusCode = HttpStatusCode(508);
    pub const NOT_EXTENDED: HttpStatusCode = HttpStatusCode(510);
    pub const NETWORK_AUTHENTICATION_REQUIRED: HttpStatusCode = HttpStatusCode(511);

    /// Returns the response's status code.
    pub fn code(&self) -> u16 {
        self.0
    }

    /// Returns the status code's description.
    pub fn description(&self) -> &'static str {
        match self.0 {
            100 => "Continue",
            101 => "Switching Protocols",
            102 => "Processing",
            103 => "Early Hints",

            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            203 => "Non-Authoritative Information",
            204 => "No Content",
            205 => "Reset Content",
            206 => "Partial Content",
            207 => "Multi-Status",
            208 => "Already Reported",
            226 => "IM Used",

            300 => "Multiple Choices",
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            304 => "Not Modified",
            305 => "Use Proxy",
            307 => "Temporary Redirecte",
            308 => "Permanent Redirect",

            400 => "Bad Request",
            401 => "Unauthorized",
            402 => "Payment Required",
            403 => "Forbidden",
            404 => "Not found",
            405 => "Method Not Allowed",
            406 => "Not Acceptable",
            407 => "Proxy Authentication Required",
            408 => "Request Timeout",
            409 => "Conflict",
            410 => "Gone",
            411 => "Length Required",
            412 => "Precondition Failed",
            413 => "Payload Too Large",
            414 => "URI Too Long",
            415 => "Unsupported Media Type",
            416 => "Range Not Satisfiable",
            417 => "Expectation Failed",
            418 => "I'm a teapot",
            421 => "Misdirected Request",
            422 => "Unprocessable Content",
            423 => "Locked",
            424 => "Failed Dependency",
            425 => "Too Early",
            426 => "Upgrade Required",
            428 => "Precondition Required",
            429 => "Too Many Requests",
            431 => "Request Header Fields Too Large",
            451 => "Unavailable For Legal Reasons",

            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            504 => "Gateway Timeout",
            505 => "HTTP Version Not Supported",
            506 => "Variant Also Negotiates",
            507 => "Insufficient Storage",
            508 => "Loop Detected",
            510 => "Not Extended",
            511 => "Network",
            _ => "Unknown",
        }
    }
}

impl fmt::Display for HttpStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.description(),)
    }
}
