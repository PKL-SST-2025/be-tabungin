use sqlx::PgPool;
use uuid::Uuid;
// use bcrypt::{hash, verify, DEFAULT_COST};
use anyhow::{Result, anyhow};

use crate::models::{User, RegisterRequest, LoginRequest, AuthResponse};
use crate::utils::jwt::generate_jwt_token;

pub async fn register_user(
    pool: &PgPool,
    request: &RegisterRequest,
) -> Result<AuthResponse> {
    // Check if user already exists
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&request.email)
    .fetch_optional(pool)
    .await?;

    if existing_user.is_some() {
        return Err(anyhow!("User with this email already exists"));
    }

    // Simpan password secara plain (TIDAK AMAN, hanya untuk testing/dev)
    let password_hash = request.password.clone();

    // Create user
    let user_id = Uuid::new_v4();
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, full_name, email, password_hash, is_admin, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        RETURNING *
        "#
    )
    .bind(user_id)
    .bind(&request.full_name)
    .bind(&request.email)
    .bind(&password_hash)
    .bind(false) // Default to non-admin
    .fetch_one(pool)
    .await?;

    // Generate JWT token
    let token = generate_jwt_token(user.id, &user.email, user.is_admin)?;

    Ok(AuthResponse {
        token,
        user: user.into(),
    })
}

pub async fn login_user(
    pool: &PgPool,
    request: &LoginRequest,
) -> Result<AuthResponse> {
    // Find user by email
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&request.email)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("Invalid email or password"))?;

    // Cek password plain (TIDAK AMAN, hanya untuk testing/dev)
    if request.password != user.password_hash {
        return Err(anyhow!("Invalid email or password"));
    }

    // Check admin login if required
    if let Some(is_admin_required) = request.is_admin {
        if is_admin_required && !user.is_admin {
            return Err(anyhow!("Access denied: Admin privileges required"));
        }
    }

    // Generate JWT token
    let token = generate_jwt_token(user.id, &user.email, user.is_admin)?;

    Ok(AuthResponse {
        token,
        user: user.into(),
    })
}
