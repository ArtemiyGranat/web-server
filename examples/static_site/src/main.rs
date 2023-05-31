use clap::Parser;
use web_server::{
    file::File, method::HttpMethod, request::HttpRequest, response::HttpResponse,
    status_code::HttpStatusCode, Server,
};

#[derive(Parser, Debug)]
struct Args {
    /// Selects a certain address
    #[clap(short, long, display_order = 1)]
    address: String,

    /// Selects a certain port
    #[clap(short, long, display_order = 2)]
    port: u16,
}

fn hello_handler(_request: HttpRequest) -> HttpResponse {
    HttpResponse::new(
        HttpStatusCode::OK,
        Vec::new(),
        File::new("static/index.html"),
    )
}

fn setup_router(server: &mut Server) {
    server.serve(HttpMethod::Get, "/", hello_handler);
}

fn main() {
    let args = Args::parse();
    let mut server = Server::new();
    setup_router(&mut server);
    server.run(&args.address, args.port);
}
