use sqlx::PgPool;
use uuid::Uuid;
use anyhow::{Result, anyhow};

use crate::models::{
    Testimoni, TestimoniWithUser, CreateTestimoniRequest, 
    UpdateTestimoniRequest, UserResponse
};

pub async fn create_testimoni(
    pool: &PgPool,
    user_id: Uuid,
    request: &CreateTestimoniRequest,
) -> Result<Testimoni> {
    let testimoni_id = Uuid::new_v4();
    let testimoni = sqlx::query_as::<_, Testimoni>(
        r#"
        INSERT INTO testimoni (id, user_id, content, rating, is_approved, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        RETURNING *
        "#
    )
    .bind(testimoni_id)
    .bind(user_id)
    .bind(&request.content)
    .bind(request.rating)
    .bind(false) // Require admin approval
    .fetch_one(pool)
    .await?;

    Ok(testimoni)
}

pub async fn get_all_testimoni(pool: &PgPool) -> Result<Vec<TestimoniWithUser>> {
    let testimoni = sqlx::query!(
        r#"
        SELECT 
            t.id, t.content, t.rating, t.is_approved, t.created_at,
            u.id as user_id, u.full_name, u.email, u.avatar, u.is_admin, u.created_at as user_created_at
        FROM testimoni t
        JOIN users u ON t.user_id = u.id
        ORDER BY t.created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    let result = testimoni
        .into_iter()
        .map(|row| TestimoniWithUser {
            id: row.id,
            content: row.content,
            rating: row.rating,
            is_approved: row.is_approved.unwrap_or(false),
            created_at: row.created_at.unwrap_or_else(|| chrono::Utc::now()),
            user: UserResponse {
                id: row.user_id,
                full_name: row.full_name,
                email: row.email,
                avatar: row.avatar,
                is_admin: row.is_admin.unwrap_or(false),
                created_at: row.user_created_at.unwrap_or_else(|| chrono::Utc::now()),
            },
        })
        .collect();

    Ok(result)
}

pub async fn get_approved_testimoni(pool: &PgPool) -> Result<Vec<TestimoniWithUser>> {
    let testimoni = sqlx::query!(
        r#"
        SELECT 
            t.id, t.content, t.rating, t.is_approved, t.created_at,
            u.id as user_id, u.full_name, u.email, u.avatar, u.is_admin, u.created_at as user_created_at
        FROM testimoni t
        JOIN users u ON t.user_id = u.id
        WHERE t.is_approved = true
        ORDER BY t.created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    let result = testimoni
        .into_iter()
        .map(|row| TestimoniWithUser {
            id: row.id,
            content: row.content,
            rating: row.rating,
            is_approved: row.is_approved.unwrap_or(false),
            created_at: row.created_at.unwrap_or_else(|| chrono::Utc::now()),
            user: UserResponse {
                id: row.user_id,
                full_name: row.full_name,
                email: row.email,
                avatar: row.avatar,
                is_admin: row.is_admin.unwrap_or(false),
                created_at: row.user_created_at.unwrap_or_else(|| chrono::Utc::now()),
            },
        })
        .collect();

    Ok(result)
}

pub async fn get_user_testimoni(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<Testimoni>> {
    let testimoni = sqlx::query_as::<_, Testimoni>(
        "SELECT * FROM testimoni WHERE user_id = $1 ORDER BY created_at DESC"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(testimoni)
}

pub async fn update_testimoni(
    pool: &PgPool,
    testimoni_id: Uuid,
    user_id: Uuid,
    request: &UpdateTestimoniRequest,
) -> Result<Testimoni> {
    // Check if testimoni exists and belongs to user (or user is admin)
    let existing = sqlx::query!(
        "SELECT user_id FROM testimoni WHERE id = $1",
        testimoni_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("Testimoni not found"))?;

    // Check ownership (this should be enhanced with admin check)
    if existing.user_id != user_id {
        return Err(anyhow!("Access denied"));
    }

    let mut query = String::from("UPDATE testimoni SET updated_at = NOW()");
    let mut params: Vec<String> = Vec::new();
    let mut param_count = 1;

    if let Some(content) = &request.content {
        query.push_str(&format!(", content = ${}", param_count));
        params.push(content.clone());
        param_count += 1;
    }

    if let Some(rating) = &request.rating {
        query.push_str(&format!(", rating = ${}", param_count));
        params.push(rating.to_string());
        param_count += 1;
    }

    if let Some(is_approved) = &request.is_approved {
        query.push_str(&format!(", is_approved = ${}", param_count));
        params.push(is_approved.to_string());
        param_count += 1;
    }

    query.push_str(&format!(" WHERE id = ${} RETURNING *", param_count));

    let mut query_builder = sqlx::query_as::<_, Testimoni>(&query);
    
    for param in params {
        query_builder = query_builder.bind(param);
    }
    query_builder = query_builder.bind(testimoni_id);

    let testimoni = query_builder
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| anyhow!("Testimoni not found"))?;

    Ok(testimoni)
}

pub async fn delete_testimoni(
    pool: &PgPool,
    testimoni_id: Uuid,
    user_id: Uuid,
) -> Result<()> {
    // Check if testimoni exists and belongs to user (or user is admin)
    let existing = sqlx::query!(
        "SELECT user_id FROM testimoni WHERE id = $1",
        testimoni_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("Testimoni not found"))?;

    // Check ownership (this should be enhanced with admin check)
    if existing.user_id != user_id {
        return Err(anyhow!("Access denied"));
    }

    sqlx::query!(
        "DELETE FROM testimoni WHERE id = $1",
        testimoni_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
