use std::net::TcpListener;

/// Spin up an instance of our application and return its address
/// (e.g. http://127.0.0.1:XXXX)
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server = velo::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    // return application address to the caller
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_test() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
