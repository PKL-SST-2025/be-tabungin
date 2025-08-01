use sqlx::PgPool;
use uuid::Uuid;
use anyhow::{Result, anyhow};
use chrono::{Utc, Duration};

use crate::models::{User, ForgotPasswordRequest, ResetPasswordRequest};
use crate::utils::jwt::generate_jwt_token;

pub async fn forgot_password(
    pool: &PgPool,
    request: &ForgotPasswordRequest,
) -> Result<String> {
    // Check if user exists
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&request.email)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("User not found"))?;

    // Generate reset token (bisa juga pakai JWT atau random string)
    let reset_token = generate_jwt_token(user.id, &user.email, user.is_admin)?;
    
    // Dalam implementasi real, Anda akan:
    // 1. Simpan token ke database dengan expiry time
    // 2. Kirim email dengan link reset password
    
    // Untuk demo, kita return token langsung
    Ok(reset_token)
}

pub async fn reset_password(
    pool: &PgPool,
    request: &ResetPasswordRequest,
) -> Result<String> {
    // Validasi password confirmation
    if request.new_password != request.confirm_password {
        return Err(anyhow!("Passwords do not match"));
    }

    // Dalam implementasi real, Anda akan:
    // 1. Validate reset token dari database
    // 2. Check expiry time
    // 3. Decode user_id dari token
    
    // Untuk demo, kita decode token JWT
    let claims = crate::utils::jwt::validate_jwt_token(&request.reset_token)?;
    
    // Update password user
    let updated_rows = sqlx::query!(
        "UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2",
        &request.new_password, // Plain password untuk demo
        claims.user_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    if updated_rows == 0 {
        return Err(anyhow!("User not found or password reset failed"));
    }

    Ok("Password reset successfully".to_string())
}

pub async fn change_password(
    pool: &PgPool,
    user_id: Uuid,
    old_password: &str,
    new_password: &str,
) -> Result<String> {
    // Get current user
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("User not found"))?;

    // Verify old password
    if old_password != user.password_hash {
        return Err(anyhow!("Current password is incorrect"));
    }

    // Update password
    sqlx::query!(
        "UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2",
        new_password,
        user_id
    )
    .execute(pool)
    .await?;

    Ok("Password changed successfully".to_string())
}
