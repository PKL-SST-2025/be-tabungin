use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use uuid::Uuid;
use chrono::Datelike;
use sqlx::PgPool;

use crate::services::reminder_service::ReminderService;
use crate::middleware::auth::AuthenticatedUser;

pub fn reminder_routes() -> actix_web::Scope {
    web::scope("/reminders")
        .route("", web::get().to(get_user_reminders))
        .route("/upcoming", web::get().to(get_upcoming_reminders))
        .route("/today", web::get().to(get_todays_reminders))
        .route("/calendar", web::get().to(get_calendar_events))
        .route("/{id}/complete", web::put().to(mark_reminder_completed))
}

pub async fn get_user_reminders(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    query: web::Query<serde_json::Value>,
) -> Result<HttpResponse> {
    let reminder_service = ReminderService::new(pool.get_ref().clone());
    let limit = query.get("limit").and_then(|v| v.as_i64()).map(|l| l as i32);

    match reminder_service.get_user_reminders(user.id, limit).await {
        Ok(reminders) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "data": reminders
        }))),
        Err(err) => {
            eprintln!("Error getting user reminders: {}", err);
            Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Failed to get reminders"
            })))
        }
    }
}

pub async fn get_upcoming_reminders(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    query: web::Query<serde_json::Value>,
) -> Result<HttpResponse> {
    let reminder_service = ReminderService::new(pool.get_ref().clone());
    let days = query.get("days").and_then(|v| v.as_i64()).map(|d| d as i32);

    match reminder_service.get_upcoming_reminders(user.id, days).await {
        Ok(reminders) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "data": reminders
        }))),
        Err(err) => {
            eprintln!("Error getting upcoming reminders: {}", err);
            Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Failed to get upcoming reminders"
            })))
        }
    }
}

pub async fn get_todays_reminders(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let reminder_service = ReminderService::new(pool.get_ref().clone());
    
    match reminder_service.get_todays_reminders(user.id).await {
        Ok(reminders) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "data": reminders
        }))),
        Err(err) => {
            eprintln!("Error getting today's reminders: {}", err);
            Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Failed to get today's reminders"
            })))
        }
    }
}

pub async fn mark_reminder_completed(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let reminder_service = ReminderService::new(pool.get_ref().clone());
    let reminder_id = path.into_inner();

    match reminder_service.mark_reminder_completed(reminder_id, user.id).await {
        Ok(true) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Reminder marked as completed"
        }))),
        Ok(false) => Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": "Reminder not found"
        }))),
        Err(err) => {
            eprintln!("Error marking reminder as completed: {}", err);
            Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Failed to mark reminder as completed"
            })))
        }
    }
}

pub async fn get_calendar_events(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    query: web::Query<serde_json::Value>,
) -> Result<HttpResponse> {
    let reminder_service = ReminderService::new(pool.get_ref().clone());
    let month = query
        .get("month")
        .and_then(|v| v.as_i64())
        .map(|m| m as i32)
        .unwrap_or_else(|| chrono::Utc::now().month() as i32);
    
    let year = query
        .get("year")
        .and_then(|v| v.as_i64())
        .map(|y| y as i32)
        .unwrap_or_else(|| chrono::Utc::now().year());

    match reminder_service.get_calendar_events(user.id, month, year).await {
        Ok(events) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "data": events
        }))),
        Err(err) => {
            eprintln!("Error getting calendar events: {}", err);
            Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Failed to get calendar events"
            })))
        }
    }
}
