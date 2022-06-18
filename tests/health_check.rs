use std::net::TcpListener;

use reqwest::{Client, StatusCode};

type BoundAddress = String;

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

fn spawn_app() -> BoundAddress {
    let address = "127.0.0.1:0";
    let tcp_listener = TcpListener::bind(address).expect("could not bind to an address");
    let socket_address = tcp_listener
        .local_addr()
        .expect("could not get socket address");
    let bound_address = socket_address.to_string();

    let server = zero2prod::run(tcp_listener).expect("Could not run a server");
    tokio::spawn(server);
    format!("http://{}", bound_address)
}
