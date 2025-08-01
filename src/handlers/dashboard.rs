use actix_web::{web, HttpResponse, Result, Scope};
use sqlx::PgPool;

use crate::services::dashboard_service::{get_dashboard_stats, get_user_analytics};
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

pub fn dashboard_routes() -> Scope {
    web::scope("/dashboard")
        .route("/stats", web::get().to(get_dashboard_stats_handler))
        .route("/analytics", web::get().to(get_user_analytics_handler))
}
