use getopts::Options;
use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::ServerBuilder;

mod error;
mod methods;
mod utils;
use methods::{Methods, RpcImpl};
fn main() {
    // command line option parse phase
    let args: Vec<String> = std::env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "show this help message");
    opts.optflag("g", "listen-global", "listen on global address");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };

    if matches.opt_present("h") {
        println!("{}", opts.usage("Usage: mynad [options]"));
        std::process::exit(0);
    }

    let listen_addr = {
        if matches.opt_present("g") {
            println!("Listening on global address. Please be careful and add firewall rules.");
            "0.0.0.0:3030".parse().unwrap()
        } else {
            "127.0.0.1:3030".parse().unwrap()
        }
    };

    // server launch phase
    run_server(listen_addr);
}

fn run_server(listen_addr: std::net::SocketAddr) {
    let mut io = IoHandler::default();
    let rpc: RpcImpl = Default::default();
    io.extend_with(rpc.to_delegate());

    let server = ServerBuilder::new(io)
        .start_http(&listen_addr)
        .expect("Server must start with no issues");

    server.wait()
}
