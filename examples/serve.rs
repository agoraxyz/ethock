use ethock_lib::server::{Entry, ServerType};

fn main() {
    Entry::new(ServerType::HTTP, "127.0.0.1:8545", "").serve();
}
