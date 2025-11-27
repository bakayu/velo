use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_returns_400_for_missing_data() {
    // Arrange
    let app_addr = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Jon%20Doe", "missing the email"),
        ("email=mail%40jondoe.com", "missing the name"),
        ("", "missing both entries"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let responses = client
            .post(format!("{}/subscribe", &app_addr.address))
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

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_case = vec![
        ("name=&email=jondoe%40gmail.com", "empty name"),
        ("name=Jon&email=", "empty email"),
        ("name=Jon&email=invalid-email", "invalid email"),
    ];

    for (body, description) in test_case {
        // Act
        let response = client
            .post(format!("{}/subscribe", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 200 OK when the payload was {}.",
            description
        )
    }
}
