use actix_web::{web, HttpResponse};
use serde;
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use tracing;

#[derive(serde::Deserialize)]
#[derive(Debug)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form:web::Form<FormData>,pool:web::Data<PgPool>) ->HttpResponse{
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "adding a new sub",
        %request_id,
        email = %form.email,
        name = %form.name
    );
    let _request_span_guard = request_span.enter();
    tracing::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name
    );
    tracing::info!(
        "request_id {} - Saving new subscriber details in the database",
        request_id
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        request_id,
        form.email,
        form.name,
        Utc::now()).execute(pool.get_ref()).await{
        Ok(_) =>{
            tracing::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }, 
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
