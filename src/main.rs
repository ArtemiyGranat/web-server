use clap::Parser;
use server::Server;

mod header;
mod response;
mod server;
mod status_code;
mod utils;
mod file_handler;
// mod request;
// mod error;
// mod router;

#[derive(Parser, Debug)]
struct Args {
    /// Selects a certain address
    #[clap(short, long, display_order = 1)]
    address: String,

    /// Selects a certain port
    #[clap(short, long, display_order = 2)]
    port: u16,
}

fn main() {
    let args = Args::parse();
    let server = Server::new(&args.address, args.port);
    server.run();
}
