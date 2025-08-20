use actix_web::{web, HttpResponse, Result, Scope};
use sqlx::PgPool;

use crate::services::dashboard_service::{
    get_dashboard_stats, get_user_analytics, get_trend_data, get_rating_distribution
};
use crate::middleware::auth::AuthenticatedUser;
use crate::utils::response::{ErrorResponse, ApiResponse};

pub async fn get_dashboard_stats_handler(
    _user: AuthenticatedUser, // Admin access
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    match get_dashboard_stats(&pool).await {
        Ok(stats) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Dashboard stats retrieved successfully".to_string(),
            data: Some(stats),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to retrieve dashboard stats".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn get_user_analytics_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    match get_user_analytics(&pool, user.id).await {
        Ok(analytics) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "User analytics retrieved successfully".to_string(),
            data: Some(analytics),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to retrieve user analytics".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn get_trend_data_handler(
    _user: AuthenticatedUser, // Admin access
    pool: web::Data<PgPool>,
    query: web::Query<serde_json::Map<String, serde_json::Value>>,
) -> Result<HttpResponse> {
    let days = query
        .get("days")
        .and_then(|v| v.as_i64())
        .unwrap_or(30) as i32;

    match get_trend_data(&pool, days).await {
        Ok(trends) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Trend data retrieved successfully".to_string(),
            data: Some(trends),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to retrieve trend data".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn get_rating_distribution_handler(
    _user: AuthenticatedUser, // Admin access
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    match get_rating_distribution(&pool).await {
        Ok(distribution) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Rating distribution retrieved successfully".to_string(),
            data: Some(distribution),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to retrieve rating distribution".to_string(),
            message: e.to_string(),
        })),
    }
}

pub fn dashboard_routes() -> Scope {
    web::scope("/dashboard")
        .route("/stats", web::get().to(get_dashboard_stats_handler))
        .route("/analytics", web::get().to(get_user_analytics_handler))
        .route("/trends", web::get().to(get_trend_data_handler))
        .route("/ratings", web::get().to(get_rating_distribution_handler))
}
