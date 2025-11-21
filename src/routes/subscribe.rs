//! # Subscribe Route
//!
//! Handles user subscriptions to the newsletter.

use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

/// Form data for the subscription endpoint.
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

/// Handler for the `POST /subscribe` endpoint.
///
/// Adds a new subscriber to the database using the provided form data.
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => {
            tracing::error!(
                "[Req ID: {}] - Failed to execute query: {:?}",
                request_id,
                error
            );
            HttpResponse::InternalServerError().finish()
        }
    };

    HttpResponse::Ok().finish()
}
