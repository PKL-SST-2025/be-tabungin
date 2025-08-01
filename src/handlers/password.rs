use actix_web::{web, HttpResponse, Result, Scope};
use sqlx::PgPool;
use validator::Validate;

use crate::models::{ForgotPasswordRequest, ResetPasswordRequest};
use crate::services::password_service::{forgot_password, reset_password, change_password};
use crate::middleware::auth::AuthenticatedUser;
use crate::utils::response::{ErrorResponse, ApiResponse};

pub async fn forgot_password_handler(
    pool: web::Data<PgPool>,
    form: web::Json<ForgotPasswordRequest>,
) -> Result<HttpResponse> {
    // Validate request
    if let Err(errors) = form.validate() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Validation failed".to_string(),
            message: format!("{:?}", errors),
        }));
    }

    match forgot_password(&pool, &form).await {
        Ok(reset_token) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Reset password email sent (demo: token returned)".to_string(),
            data: Some(serde_json::json!({ "reset_token": reset_token })),
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Forgot password failed".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn reset_password_handler(
    pool: web::Data<PgPool>,
    form: web::Json<ResetPasswordRequest>,
) -> Result<HttpResponse> {
    // Validate request
    if let Err(errors) = form.validate() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Validation failed".to_string(),
            message: format!("{:?}", errors),
        }));
    }

    match reset_password(&pool, &form).await {
        Ok(message) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message,
            data: None::<()>,
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Reset password failed".to_string(),
            message: e.to_string(),
        })),
    }
}

#[derive(serde::Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

pub async fn change_password_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    form: web::Json<ChangePasswordRequest>,
) -> Result<HttpResponse> {
    match change_password(&pool, user.id, &form.old_password, &form.new_password).await {
        Ok(message) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message,
            data: None::<()>,
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Change password failed".to_string(),
            message: e.to_string(),
        })),
    }
}

pub fn password_routes() -> Scope {
    web::scope("/password")
        .route("/forgot", web::post().to(forgot_password_handler))
        .route("/reset", web::post().to(reset_password_handler))
        .route("/change", web::post().to(change_password_handler))
}
