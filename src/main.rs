use std::net::TcpListener;

use sqlx::PgPool;

use zero2prod::configuration;
use zero2prod::startup;
use zero2prod::telemetry;

use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber =
        telemetry::make_tracing_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::read_configuration().expect("Failed to read configuration.");
    let db_connection_pool = PgPool::connect_lazy(
        configuration
            .database
            .as_connection_string()
            .expose_secret(),
    )
    .expect("Could not connect to the database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let tcp_listener = TcpListener::bind(address).expect("could not bind to an address");
    startup::run(tcp_listener, db_connection_pool)?.await
}
