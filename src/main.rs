use std::net::TcpListener;

use sqlx::PgPool;

use zero2prod::configuration;
use zero2prod::startup;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::read_configuration().expect("Failed to read configuration.");
    let db_connection_pool = PgPool::connect(&configuration.database.as_connection_string())
        .await
        .expect("Could not connect to the database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let tcp_listener = TcpListener::bind(address).expect("could not bind to an address");
    startup::run(tcp_listener, db_connection_pool)?.await
}
