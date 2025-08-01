use actix_web::{web, HttpResponse, Result, Scope};
use sqlx::PgPool;
use validator::Validate;

use crate::models::{RegisterRequest, LoginRequest};
use crate::services::auth_service::{register_user, login_user};
use crate::utils::response::{ErrorResponse, ApiResponse};

pub async fn register_handler(
    pool: web::Data<PgPool>,
    form: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    // Validate request
    if let Err(errors) = form.validate() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Validation failed".to_string(),
            message: format!("{:?}", errors),
        }));
    }

    // Check if passwords match
    if form.password != form.confirm_password {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Password mismatch".to_string(),
            message: "Password and confirm password do not match".to_string(),
        }));
    }

    match register_user(&pool, &form).await {
        Ok(auth_response) => Ok(HttpResponse::Created().json(ApiResponse {
            success: true,
            message: "User registered successfully".to_string(),
            data: Some(auth_response),
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Registration failed".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn login_handler(
    pool: web::Data<PgPool>,
    form: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    // Validate request
    if let Err(errors) = form.validate() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Validation failed".to_string(),
            message: format!("{:?}", errors),
        }));
    }

    match login_user(&pool, &form).await {
        Ok(auth_response) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Login successful".to_string(),
            data: Some(auth_response),
        })),
        Err(e) => Ok(HttpResponse::Unauthorized().json(ErrorResponse {
            error: "Login failed".to_string(),
            message: e.to_string(),
        })),
    }
}

pub fn auth_routes() -> Scope {
    web::scope("/auth")
        .route("/register", web::post().to(register_handler))
        .route("/login", web::post().to(login_handler))
}
