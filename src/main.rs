use std::net::TcpListener;

use zero2prod::configuration;
use zero2prod::startup;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::read_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let tcp_listener = TcpListener::bind(address).expect("could not bind to an address");
    startup::run(tcp_listener)?.await
}
