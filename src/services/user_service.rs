use sqlx::PgPool;
use uuid::Uuid;
use anyhow::{Result, anyhow};

use crate::models::{User, UserResponse, UpdateUserRequest};

pub async fn get_user_profile(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<UserResponse> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("User not found"))?;

    Ok(user.into())
}

pub async fn update_user_profile(
    pool: &PgPool,
    user_id: Uuid,
    request: &UpdateUserRequest,
) -> Result<UserResponse> {
    let mut query = String::from("UPDATE users SET updated_at = NOW()");
    let mut params: Vec<String> = Vec::new();
    let mut param_count = 1;

    if let Some(full_name) = &request.full_name {
        query.push_str(&format!(", full_name = ${}", param_count));
        params.push(full_name.clone());
        param_count += 1;
    }
    if let Some(avatar) = &request.avatar {
        query.push_str(&format!(", avatar = ${}", param_count));
        params.push(avatar.clone());
        param_count += 1;
    }
    if let Some(nomor_telepon) = &request.nomor_telepon {
        query.push_str(&format!(", nomor_telepon = ${}", param_count));
        params.push(nomor_telepon.clone());
        param_count += 1;
    }
    if let Some(alamat) = &request.alamat {
        query.push_str(&format!(", alamat = ${}", param_count));
        params.push(alamat.clone());
        param_count += 1;
    }
    if let Some(posisi_jabatan) = &request.posisi_jabatan {
        query.push_str(&format!(", posisi_jabatan = ${}", param_count));
        params.push(posisi_jabatan.clone());
        param_count += 1;
    }
    query.push_str(&format!(" WHERE id = ${} RETURNING *", param_count));

    let mut query_builder = sqlx::query_as::<_, User>(&query);
    for param in params {
        query_builder = query_builder.bind(param);
    }
    query_builder = query_builder.bind(user_id);

    let user = query_builder
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| anyhow!("User not found"))?;

    Ok(user.into())
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<UserResponse>> {
    let users = sqlx::query_as::<_, User>(
        "SELECT * FROM users ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(users.into_iter().map(|user| user.into()).collect())
}
