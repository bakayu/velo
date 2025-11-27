use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let test_app = spawn_app().await;

    // Act
    let body = "name=Jon%20Doe&email=mail%40jondoe.com";
    let response = test_app.post_subscription(body.into()).await;

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "mail@jondoe.com");
    assert_eq!(saved.name, "Jon Doe");
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_data() {
    // Arrange
    let test_app = spawn_app().await;
    let test_cases = vec![
        ("name=Jon%20Doe", "missing the email"),
        ("email=mail%40jondoe.com", "missing the name"),
        ("", "missing both entries"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let responses = test_app.post_subscription(invalid_body.into()).await;

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
    let test_app = spawn_app().await;
    let test_case = vec![
        ("name=&email=jondoe%40gmail.com", "empty name"),
        ("name=Jon&email=", "empty email"),
        ("name=Jon&email=invalid-email", "invalid email"),
    ];

    for (body, description) in test_case {
        // Act
        let response = test_app.post_subscription(body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 200 OK when the payload was {}.",
            description
        )
    }
}
