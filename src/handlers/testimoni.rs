use actix_web::{web, HttpResponse, Result, Scope};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::models::{CreateTestimoniRequest, UpdateTestimoniRequest};
use crate::services::testimoni_service::{
    create_testimoni, get_all_testimoni, get_user_testimoni, 
    update_testimoni, delete_testimoni, get_approved_testimoni
};
use crate::middleware::auth::AuthenticatedUser;
use crate::utils::response::{ErrorResponse, ApiResponse};

pub async fn create_testimoni_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    form: web::Json<CreateTestimoniRequest>,
) -> Result<HttpResponse> {
    // Validate request
    if let Err(errors) = form.validate() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Validation failed".to_string(),
            message: format!("{:?}", errors),
        }));
    }

    match create_testimoni(&pool, user.id, &form).await {
        Ok(testimoni) => Ok(HttpResponse::Created().json(ApiResponse {
            success: true,
            message: "Testimoni created successfully".to_string(),
            data: Some(testimoni),
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Failed to create testimoni".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn get_all_testimoni_handler(
    _user: AuthenticatedUser, // Admin access
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    match get_all_testimoni(&pool).await {
        Ok(testimoni) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Testimoni retrieved successfully".to_string(),
            data: Some(testimoni),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to retrieve testimoni".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn get_approved_testimoni_handler(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    match get_approved_testimoni(&pool).await {
        Ok(testimoni) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Approved testimoni retrieved successfully".to_string(),
            data: Some(testimoni),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to retrieve testimoni".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn get_user_testimoni_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    match get_user_testimoni(&pool, user.id).await {
        Ok(testimoni) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "User testimoni retrieved successfully".to_string(),
            data: Some(testimoni),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to retrieve testimoni".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn update_testimoni_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    form: web::Json<UpdateTestimoniRequest>,
) -> Result<HttpResponse> {
    let testimoni_id = path.into_inner();

    match update_testimoni(&pool, testimoni_id, user.id, &form).await {
        Ok(testimoni) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Testimoni updated successfully".to_string(),
            data: Some(testimoni),
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Failed to update testimoni".to_string(),
            message: e.to_string(),
        })),
    }
}

pub async fn delete_testimoni_handler(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let testimoni_id = path.into_inner();

    match delete_testimoni(&pool, testimoni_id, user.id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::<()> {
            success: true,
            message: "Testimoni deleted successfully".to_string(),
            data: None,
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "Failed to delete testimoni".to_string(),
            message: e.to_string(),
        })),
    }
}

pub fn testimoni_routes() -> Scope {
    web::scope("/testimoni")
        .route("", web::post().to(create_testimoni_handler))
        .route("/all", web::get().to(get_all_testimoni_handler))
        .route("/approved", web::get().to(get_approved_testimoni_handler))
        .route("/my", web::get().to(get_user_testimoni_handler))
        .route("/{id}", web::put().to(update_testimoni_handler))
        .route("/{id}", web::delete().to(delete_testimoni_handler))
}
