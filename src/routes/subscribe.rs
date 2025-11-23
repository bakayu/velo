//! # Subscribe Route
//!
//! Handles user subscriptions to the newsletter.

use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

/// Form data for the subscription endpoint.
///
/// This struct represents the raw data received from the HTML form.
#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    /// Converts raw form data into a validated `NewSubscriber`.
    ///
    /// Returns an error if either the name or email fails validation.
    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;

        Ok(Self { email, name })
    }
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
/// Handler for the `POST /subscribe` endpoint.
///
/// # Flow
/// 1. Parses the form data.
/// 2. Validates the input (name and email format).
/// 3. Inserts the subscriber into the database.
///
/// # Returns
/// - `200 OK` if the subscription was successful.
/// - `400 Bad Request` if the input data is invalid.
/// - `500 Internal Server Error` if the database insertion fails.
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_subscriber = match form.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
/// Inserts a new subscriber into the `subscriptions` table.
///
/// Generates a unique UUID and captures the current UTC time for the subscription.
///
/// # Arguments
/// * `pool` - The database connection pool.
/// * `new_subscriber` - The validated subscriber domain object.
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
