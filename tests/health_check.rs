use reqwest::{Client, StatusCode};

#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute the request");

    assert!(response.status() == StatusCode::OK);
}

fn spawn_app() {
    let server = zero2prod::run().expect("Could not run a server");
    tokio::spawn(server);
}
