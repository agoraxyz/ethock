use ethock_lib::server::{Entry, ServerType};

fn main() {
    let mut addr: &str = "localhost:8545";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        addr = &args[1][..];
    }
    Entry::new(ServerType::HTTP, addr, "debug").serve();
}
