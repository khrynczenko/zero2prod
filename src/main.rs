use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8000";
    let tcp_listener = TcpListener::bind(address).expect("could not bind to an address");
    run(tcp_listener)?.await
}
