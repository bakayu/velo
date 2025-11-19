use std::net::TcpListener;

/// Spin up an instance of our application and return its address
/// (e.g. http://127.0.0.1:XXXX)
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server = velo::run(listener).expect("Failed to bind address");
    drop(tokio::spawn(server));

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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_addr = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=Jon%20Doe&email=mail%40jondoe.com";
    let response = client
        .post(format!("{}/subscribe", &app_addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute post request");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_data() {
    // Arrange
    let app_addr = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Jon%20Doe", "missing the email"),
        ("email=mail%40jondoe.com", "missing the name"),
        ("", "missing both entries"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let responses = client
            .post(format!("{}/subscribe", &app_addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute post request");

        // Assert
        assert_eq!(
            400,
            responses.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
