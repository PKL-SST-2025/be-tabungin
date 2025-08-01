use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
    pub avatar: Option<String>,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub avatar: Option<String>,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Testimoni {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub rating: i32,
    pub is_approved: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestimoniWithUser {
    pub id: Uuid,
    pub content: String,
    pub rating: i32,
    pub is_approved: bool,
    pub created_at: DateTime<Utc>,
    pub user: UserResponse,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 2, max = 100, message = "Full name must be between 2 and 100 characters"))]
    pub full_name: String,
    
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
    
    pub confirm_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
    
    pub is_admin: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTestimoniRequest {
    #[validate(length(min = 10, max = 1000, message = "Content must be between 10 and 1000 characters"))]
    pub content: String,
    
    #[validate(range(min = 1, max = 5, message = "Rating must be between 1 and 5"))]
    pub rating: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTestimoniRequest {
    pub content: Option<String>,
    pub rating: Option<i32>,
    pub is_approved: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub full_name: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub new_password: String,
    
    pub confirm_password: String,
    pub reset_token: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            full_name: user.full_name,
            email: user.email,
            avatar: user.avatar,
            is_admin: user.is_admin,
            created_at: user.created_at,
        }
    }
}
