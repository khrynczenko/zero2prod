use sqlx::PgPool;

use std::net::TcpListener;

use zero2prod::configuration;
use zero2prod::startup;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = configuration::get_configuration().expect("Failed to read configuration");
    let db_connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Posgres");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("Could not bind on given address.");
    startup::run(listener, db_connection_pool)?.await
}
