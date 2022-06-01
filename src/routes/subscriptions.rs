//! src/routes/subscription.rs

use actix_web::HttpResponse;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}
