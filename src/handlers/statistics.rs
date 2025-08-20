use actix_web::{web, HttpResponse, Result, Scope};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};

use crate::services::statistics_service::{
    get_user_statistics, get_user_achievements, get_user_streak_data
};
use crate::middleware::auth::AuthenticatedUser;
use crate::utils::response::{ErrorResponse, ApiResponse};

#[derive(Deserialize)]
pub struct StreakQuery {
    days: Option<i32>,
}

pub async fn get_user_statistics_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    match get_user_statistics(&pool, user.id).await {
        Ok(stats) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "User statistics retrieved successfully".to_string(),
            data: Some(stats),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to retrieve user statistics".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn get_user_achievements_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    match get_user_achievements(&pool, user.id).await {
        Ok(achievements) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "User achievements retrieved successfully".to_string(),
            data: Some(achievements),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to retrieve user achievements".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn get_user_streak_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    query: web::Query<StreakQuery>,
) -> Result<HttpResponse> {
    let days = query.days.unwrap_or(16);
    match get_user_streak_data(&pool, user.id, days).await {
        Ok(streak_data) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "User streak data retrieved successfully".to_string(),
            data: Some(streak_data),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to retrieve user streak data".to_string(),
            message: e.to_string(),
        })),
    }
}

pub fn statistics_routes() -> Scope {
    web::scope("/statistics")
        .route("", web::get().to(get_user_statistics_handler))
        .route("/achievements", web::get().to(get_user_achievements_handler))
        .route("/streak", web::get().to(get_user_streak_handler))
}
