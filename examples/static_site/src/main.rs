use clap::Parser;
use web_server::{request::Request, response::Response, Server};

#[derive(Parser, Debug)]
struct Args {
    /// Selects a certain address
    #[clap(short, long, display_order = 1)]
    address: String,

    /// Selects a certain port
    #[clap(short, long, display_order = 2)]
    port: u16,
}

fn hello_handler(request: Request) -> Response {
    Response::new(200, Vec::new(), "Hello world".to_string())
}

fn main() {
    let args = Args::parse();
    let mut server = Server::new();
    server.serve("GET", "/", hello_handler);
    server.run(&args.address, args.port);
}
