use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;

use zero2prod::configuration;
use zero2prod::startup;
use zero2prod::telemetry;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber =
        telemetry::make_tracing_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::read_configuration().expect("Failed to read configuration.");
    let db_connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let tcp_listener = TcpListener::bind(address).expect("could not bind to an address");
    startup::run(tcp_listener, db_connection_pool)?.await
}
