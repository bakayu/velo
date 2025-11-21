//! # Health Check Route
//!
//! Simple endpoint to verify service availability.

use actix_web::HttpResponse;

/// Handler for the `GET /health_check` endpoint.
///
/// Returns a 200 OK response to indicate the server is running.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
