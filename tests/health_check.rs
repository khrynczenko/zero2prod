use std::net::TcpListener;

use reqwest::{Client, StatusCode};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use zero2prod::configuration::read_configuration;
use zero2prod::configuration::DatabaseSettings;

const APPLICATION_X_WWW_FORM_URL_ENCODED: &'static str = "application/x-www-form-urlencoded";

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_app().await;
    let endpoint_address = format!("{}/health_check", test_app.address);

    let client = Client::new();

    let response = client
        .get(endpoint_address)
        .send()
        .await
        .expect("Failed to execute the request");

    assert!(response.status() == StatusCode::OK);
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", &test_app.address))
        .body(body)
        .header("Content-Type", APPLICATION_X_WWW_FORM_URL_ENCODED)
        .send()
        .await
        .expect("failed to execute a request");

    assert_eq!(response.status().as_u16(), 200);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, reason) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &test_app.address))
            .body(invalid_body)
            .header("Content-Type", APPLICATION_X_WWW_FORM_URL_ENCODED)
            .send()
            .await
            .expect("failed to execute a request");

        assert_eq!(
            response.status().as_u16(),
            400,
            "API did not fail with 400 Bad Reqest even though it should. The paylod was {}.",
            reason
        );
    }
}

async fn spawn_app() -> TestApp {
    let mut configuration = read_configuration().expect("Could not read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let address = "127.0.0.1:0";
    let tcp_listener = TcpListener::bind(address).expect("could not bind to an address");
    let socket_address = tcp_listener
        .local_addr()
        .expect("could not get socket address");
    let bound_address = socket_address.to_string();

    let connection_pool = configure_database(&configuration.database).await;

    let server = zero2prod::startup::run(tcp_listener, connection_pool.clone())
        .expect("Could not run a server");
    tokio::spawn(server);
    TestApp {
        address: format!("http://{}", bound_address),
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.as_connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.as_connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
