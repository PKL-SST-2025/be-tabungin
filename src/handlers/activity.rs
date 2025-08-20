use actix_web::{web, HttpResponse, Result, Scope};
use sqlx::PgPool;
use validator::Validate;

use crate::services::activity_service::{
    create_activity, get_user_activities, get_recent_activities_for_dashboard
};
use crate::middleware::auth::AuthenticatedUser;
use crate::models::CreateActivityRequest;
use crate::utils::response::{ErrorResponse, ApiResponse};

pub async fn create_activity_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    req: web::Json<CreateActivityRequest>,
) -> Result<HttpResponse> {
    if let Err(errors) = req.validate() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Validation failed".to_string(),
            message: format!("{:?}", errors),
        }));
    }

    match create_activity(&pool, user.id, req.into_inner()).await {
        Ok(activity) => Ok(HttpResponse::Created().json(ApiResponse {
            success: true,
            message: "Activity created successfully".to_string(),
            data: Some(activity),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to create activity".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn get_user_activities_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    query: web::Query<serde_json::Map<String, serde_json::Value>>,
) -> Result<HttpResponse> {
    let limit = query
        .get("limit")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);

    match get_user_activities(&pool, user.id, limit).await {
        Ok(activities) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Activities retrieved successfully".to_string(),
            data: Some(activities),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to retrieve activities".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn get_recent_activities_handler(
    _user: AuthenticatedUser, // Admin access
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    match get_recent_activities_for_dashboard(&pool).await {
        Ok(activities) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Recent activities retrieved successfully".to_string(),
            data: Some(activities),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to retrieve recent activities".to_string(),
            message: e.to_string(),
        })),
    }
}

pub fn activity_routes() -> Scope {
    web::scope("/activities")
        .route("", web::post().to(create_activity_handler))
        .route("", web::get().to(get_user_activities_handler))
        .route("/recent", web::get().to(get_recent_activities_handler))
}
