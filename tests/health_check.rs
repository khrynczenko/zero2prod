use std::net::TcpListener;

use reqwest::{Client, StatusCode};
use sqlx::{PgConnection, Connection};

use zero2prod::configuration::read_configuration;

const APPLICATION_X_WWW_FORM_URL_ENCODED: &'static str = "application/x-www-form-urlencoded";

#[tokio::test]
async fn health_check_works() {
    let bound_address = spawn_app();
    let endpoint_address = format!("{}/health_check", bound_address);

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
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let configuration = read_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.as_connection_string();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .body(body)
        .header("Content-Type", APPLICATION_X_WWW_FORM_URL_ENCODED)
        .send()
        .await
        .expect("failed to execute a request");

    assert_eq!(response.status().as_u16(), 200);

    let mut db_connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut db_connection)
        .await
        .expect("Failed to fetch saved subscription");
    assert_eq!(saved.email,"ursula_le_guin%40gmail.com");
    assert_eq!(saved.name,"le%20guin");
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, reason) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
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

fn spawn_app() -> String {
    let address = "127.0.0.1:0";
    let tcp_listener = TcpListener::bind(address).expect("could not bind to an address");
    let socket_address = tcp_listener
        .local_addr()
        .expect("could not get socket address");
    let bound_address = socket_address.to_string();

    let server = zero2prod::startup::run(tcp_listener).expect("Could not run a server");
    tokio::spawn(server);
    format!("http://{}", bound_address)
}
