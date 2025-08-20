use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::Notification;

#[get("/notifications/{user_id}")]
pub async fn get_notifications(
    user_id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match sqlx::query_as!(
        Notification,
        r#"SELECT id, user_id, type, message, read, timestamp FROM notifications WHERE user_id = $1 ORDER BY timestamp DESC"#,
        *user_id
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(notifs) => {
            let notif_responses: Vec<_> = notifs.into_iter().map(|n| {
                serde_json::json!({
                    "id": n.id,
                    "user_id": n.user_id,
                    "type": n.r#type,
                    "message": n.message,
                    "read": n.read,
                    "timestamp": chrono::DateTime::<chrono::Utc>::from_utc(n.timestamp, chrono::Utc)
                })
            }).collect();
            HttpResponse::Ok().json(notif_responses)
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[post("/notifications/{user_id}")]
pub async fn add_notification(
    user_id: web::Path<i64>,
    notif: web::Json<Notification>,
) -> impl Responder {
    // TODO: Insert to DB
    HttpResponse::Ok().json(notif.0)
}

#[post("/notifications/read/{notif_id}")]
pub async fn mark_as_read(notif_id: web::Path<i64>) -> impl Responder {
    // TODO: Update DB
    HttpResponse::Ok().body("Marked as read")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_notifications)
        .service(add_notification)
        .service(mark_as_read);
}
