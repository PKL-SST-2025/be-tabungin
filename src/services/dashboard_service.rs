use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_users: i64,
    pub total_testimoni: i64,
    pub approved_testimoni: i64,
    pub pending_testimoni: i64,
    pub avg_rating: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAnalytics {
    pub user_testimoni_count: i64,
    pub user_avg_rating: Option<f64>,
    pub account_created: chrono::DateTime<chrono::Utc>,
}

pub async fn get_dashboard_stats(pool: &PgPool) -> Result<DashboardStats> {
    let stats = sqlx::query!(
        r#"
        SELECT 
            (SELECT COUNT(*) FROM users) as total_users,
            (SELECT COUNT(*) FROM testimoni) as total_testimoni,
            (SELECT COUNT(*) FROM testimoni WHERE is_approved = true) as approved_testimoni,
            (SELECT COUNT(*) FROM testimoni WHERE is_approved = false) as pending_testimoni,
            (SELECT AVG(rating::float) FROM testimoni WHERE is_approved = true) as avg_rating
        "#
    )
    .fetch_one(pool)
    .await?;

    Ok(DashboardStats {
        total_users: stats.total_users.unwrap_or(0),
        total_testimoni: stats.total_testimoni.unwrap_or(0),
        approved_testimoni: stats.approved_testimoni.unwrap_or(0),
        pending_testimoni: stats.pending_testimoni.unwrap_or(0),
        avg_rating: stats.avg_rating,
    })
}

pub async fn get_user_analytics(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<UserAnalytics> {
    let analytics = sqlx::query!(
        r#"
        SELECT 
            (SELECT COUNT(*) FROM testimoni WHERE user_id = $1) as user_testimoni_count,
            (SELECT AVG(rating::float) FROM testimoni WHERE user_id = $1) as user_avg_rating,
            (SELECT created_at FROM users WHERE id = $1) as account_created
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(UserAnalytics {
        user_testimoni_count: analytics.user_testimoni_count.unwrap_or(0),
        user_avg_rating: analytics.user_avg_rating,
        account_created: analytics.account_created.unwrap_or_else(|| chrono::Utc::now()),
    })
}
