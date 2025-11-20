use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    log::info!(
        "[Req ID: {}] - Attempting adding '{}' '{}' as a new subscriber",
        request_id,
        form.email,
        form.name
    );
    log::info!(
        "[Req ID: {}] - Attempting saving new subscriber details to the database",
        request_id
    );
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
    .await
    {
        Ok(_) => {
            log::info!(
                "[Req ID: {}] - New subscriber details saved to the database successfully",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(error) => {
            log::error!(
                "[Req ID: {}] - Failed to execute query: {:?}",
                request_id,
                error
            );
            HttpResponse::InternalServerError().finish()
        }
    };

    HttpResponse::Ok().finish()
}
