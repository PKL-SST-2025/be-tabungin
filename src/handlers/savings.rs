use actix_web::{web, HttpResponse, Result, Scope};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::services::savings_service::{
    create_savings_target, get_user_savings_targets, get_savings_target_by_id,
    update_savings_target, delete_savings_target, add_deposit_to_target
};
use crate::services::activity_service::{
    create_deposit_activity, create_target_created_activity
};
use crate::services::statistics_service::update_user_statistics_after_deposit;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::{CreateSavingsTargetRequest, UpdateSavingsTargetRequest};
use crate::utils::response::{ErrorResponse, ApiResponse};

pub async fn create_savings_target_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    req: web::Json<CreateSavingsTargetRequest>,
) -> Result<HttpResponse> {
    if let Err(errors) = req.validate() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Validation failed".to_string(),
            message: format!("{:?}", errors),
        }));
    }

    match create_savings_target(&pool, user.id, req.into_inner()).await {
        Ok(target) => {
            // Activity creation is already handled in the service layer
            
            Ok(HttpResponse::Created().json(ApiResponse {
                success: true,
                message: "Savings target created successfully".to_string(),
                data: Some(target),
            }))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to create savings target".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn get_user_savings_targets_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    match get_user_savings_targets(&pool, user.id).await {
        Ok(targets) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Savings targets retrieved successfully".to_string(),
            data: Some(targets),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to retrieve savings targets".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn get_savings_target_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let target_id = path.into_inner();

    match get_savings_target_by_id(&pool, user.id, target_id).await {
        Ok(target) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Savings target retrieved successfully".to_string(),
            data: Some(target),
        })),
        Err(e) => Ok(HttpResponse::NotFound().json(ErrorResponse {
            error: "Savings target not found".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn update_savings_target_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateSavingsTargetRequest>,
) -> Result<HttpResponse> {
    let target_id = path.into_inner();

    match update_savings_target(&pool, user.id, target_id, req.into_inner()).await {
        Ok(target) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Savings target updated successfully".to_string(),
            data: Some(target),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to update savings target".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn delete_savings_target_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let target_id = path.into_inner();

    match delete_savings_target(&pool, user.id, target_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Savings target deleted successfully".to_string(),
            data: Some(()),
        })),
        Err(e) => Ok(HttpResponse::NotFound().json(ErrorResponse {
            error: "Failed to delete savings target".to_string(),
            message: e.to_string(),
        })),
    }
}

#[derive(serde::Deserialize)]
pub struct DepositRequest {
    pub amount: f64,
}

pub async fn add_deposit_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    req: web::Json<DepositRequest>,
) -> Result<HttpResponse> {
    let target_id = path.into_inner();
    let amount = req.amount;

    if amount <= 0.0 {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid amount".to_string(),
            message: "Deposit amount must be greater than 0".to_string(),
        }));
    }

    match add_deposit_to_target(&pool, user.id, target_id, amount).await {
        Ok(target) => {
            // Activity and statistics are already handled in the service layer
            // No need to duplicate them here
            
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                message: "Deposit added successfully".to_string(),
                data: Some(target),
            }))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to add deposit".to_string(),
            message: e.to_string(),
        })),
    }
}

pub fn savings_routes() -> Scope {
    web::scope("/savings")
        .route("/targets", web::post().to(create_savings_target_handler))
        .route("/targets", web::get().to(get_user_savings_targets_handler))
        .route("/targets/{id}", web::get().to(get_savings_target_handler))
        .route("/targets/{id}", web::put().to(update_savings_target_handler))
        .route("/targets/{id}", web::delete().to(delete_savings_target_handler))
        .route("/targets/{id}/deposit", web::post().to(add_deposit_handler))
}
