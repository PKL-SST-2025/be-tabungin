use actix_web::{web, HttpResponse, Result, Scope};
use sqlx::PgPool;
use crate::models::UpdateUserRequest;
use crate::services::user_service::{get_user_profile, update_user_profile, get_all_users};
use crate::middleware::auth::AuthenticatedUser;
use crate::utils::response::{ErrorResponse, ApiResponse};

pub async fn get_profile_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    match get_user_profile(&pool, user.id).await {
        Ok(user_profile) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Profile retrieved successfully".to_string(),
            data: Some(user_profile),
        })),
        Err(e) => Ok(HttpResponse::NotFound().json(ErrorResponse {
            error: "User not found".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn update_profile_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    form: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse> {
    match update_user_profile(&pool, user.id, &form).await {
        Ok(updated_user) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Profile updated successfully".to_string(),
            data: Some(updated_user),
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Update failed".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn get_users_handler(
    _user: AuthenticatedUser, // Admin middleware will be added later
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    match get_all_users(&pool).await {
        Ok(users) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Users retrieved successfully".to_string(),
            data: Some(users),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to retrieve users".to_string(),
            message: e.to_string(),
        })),
    }
}

pub fn user_routes() -> Scope {
    web::scope("/users")
        .route("/profile", web::get().to(get_profile_handler))
        .route("/profile", web::put().to(update_profile_handler))
        .route("", web::get().to(get_users_handler))
}
