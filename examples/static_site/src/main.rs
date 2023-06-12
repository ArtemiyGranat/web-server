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

fn hello_handler(_: HttpRequest) -> HttpResponse {
    HttpResponse::new(HttpStatusCode::OK).with_body(File::new("static/index.html"))
}

fn not_found_handler(_: HttpRequest) -> HttpResponse {
    HttpResponse::new(HttpStatusCode::NOT_FOUND).with_body(File::new("static/404.html"))
}

fn main() {
    let args = Args::parse();
    Server::new()
        .serve(HttpMethod::Get, "/{id}", hello_handler)
        .serve(HttpMethod::Get, "/find_me", not_found_handler)
        .run(&args.address, args.port)
}
