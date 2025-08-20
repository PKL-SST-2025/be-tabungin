use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use bigdecimal::BigDecimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_users: i64,
    pub total_testimoni: i64,
    pub approved_testimoni: i64,
    pub pending_testimoni: i64,
    pub avg_rating: Option<f64>,
    pub total_savings_targets: i64,
    pub completed_targets: i64,
    pub total_saved_amount: f64,
    pub total_activities: i64,
    pub active_users_today: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAnalytics {
    pub user_testimoni_count: i64,
    pub user_avg_rating: Option<f64>,
    pub account_created: chrono::DateTime<chrono::Utc>,
    pub total_savings_targets: i64,
    pub completed_targets: i64,
    pub total_saved: f64,
    pub streak_days: i32,
    pub achievements_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendData {
    pub date: chrono::NaiveDate,
    pub user_count: i64,
    pub savings_amount: f64,
    pub transactions_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RatingDistribution {
    pub rating: i32,
    pub count: i64,
    pub percentage: f64,
}

pub async fn get_dashboard_stats(pool: &PgPool) -> Result<DashboardStats> {
    let stats = sqlx::query!(
        r#"
        SELECT 
            (SELECT COUNT(*) FROM users) as total_users,
            (SELECT COUNT(*) FROM testimoni) as total_testimoni,
            (SELECT COUNT(*) FROM testimoni WHERE is_approved = true) as approved_testimoni,
            (SELECT COUNT(*) FROM testimoni WHERE is_approved = false) as pending_testimoni,
            (SELECT AVG(rating::float) FROM testimoni WHERE is_approved = true) as avg_rating,
            (SELECT COUNT(*) FROM savings_targets) as total_savings_targets,
            (SELECT COUNT(*) FROM savings_targets WHERE is_completed = true) as completed_targets,
            (SELECT COALESCE(SUM(total_saved), 0) FROM user_statistics) as total_saved_amount,
            (SELECT COUNT(*) FROM activities) as total_activities,
            (SELECT COUNT(DISTINCT user_id) FROM activities WHERE created_at::date = CURRENT_DATE) as active_users_today
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
        total_savings_targets: stats.total_savings_targets.unwrap_or(0),
        completed_targets: stats.completed_targets.unwrap_or(0),
        total_saved_amount: stats.total_saved_amount.and_then(|d| d.to_string().parse().ok()).unwrap_or(0.0),
        total_activities: stats.total_activities.unwrap_or(0),
        active_users_today: stats.active_users_today.unwrap_or(0),
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
            (SELECT created_at FROM users WHERE id = $1) as account_created,
            (SELECT COUNT(*) FROM savings_targets WHERE user_id = $1) as total_savings_targets,
            (SELECT COUNT(*) FROM savings_targets WHERE user_id = $1 AND is_completed = true) as completed_targets,
            (SELECT COALESCE(total_saved, 0) FROM user_statistics WHERE user_id = $1) as total_saved,
            (SELECT COALESCE(streak_days, 0) FROM user_statistics WHERE user_id = $1) as streak_days,
            (SELECT COALESCE(achievements_count, 0) FROM user_statistics WHERE user_id = $1) as achievements_count
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(UserAnalytics {
        user_testimoni_count: analytics.user_testimoni_count.unwrap_or(0),
        user_avg_rating: analytics.user_avg_rating,
        account_created: analytics.account_created.unwrap_or_else(|| chrono::Utc::now()),
        total_savings_targets: analytics.total_savings_targets.unwrap_or(0),
        completed_targets: analytics.completed_targets.unwrap_or(0),
        total_saved: analytics.total_saved.and_then(|d| d.to_string().parse().ok()).unwrap_or(0.0),
        streak_days: analytics.streak_days.unwrap_or(0),
        achievements_count: analytics.achievements_count.unwrap_or(0),
    })
}

pub async fn get_trend_data(pool: &PgPool, days: i32) -> Result<Vec<TrendData>> {
    let trends = sqlx::query!(
        r#"
        WITH date_series AS (
            SELECT generate_series(
                CURRENT_DATE - INTERVAL '1 day' * $1,
                CURRENT_DATE,
                INTERVAL '1 day'
            )::date as date
        )
        SELECT 
            ds.date,
            COALESCE(u.user_count, 0) as user_count,
            COALESCE(s.savings_amount, 0) as savings_amount,
            COALESCE(a.transactions_count, 0) as transactions_count
        FROM date_series ds
        LEFT JOIN (
            SELECT created_at::date as date, COUNT(*) as user_count
            FROM users
            WHERE created_at::date >= CURRENT_DATE - INTERVAL '1 day' * $2
            GROUP BY created_at::date
        ) u ON ds.date = u.date
        LEFT JOIN (
            SELECT created_at::date as date, COALESCE(SUM(amount), 0) as savings_amount
            FROM activities
            WHERE activity_type = 'deposit' AND created_at::date >= CURRENT_DATE - INTERVAL '1 day' * $3
            GROUP BY created_at::date
        ) s ON ds.date = s.date
        LEFT JOIN (
            SELECT created_at::date as date, COUNT(*) as transactions_count
            FROM activities
            WHERE activity_type = 'deposit' AND created_at::date >= CURRENT_DATE - INTERVAL '1 day' * $4
            GROUP BY created_at::date
        ) a ON ds.date = a.date
        ORDER BY ds.date
        "#,
        days as f64,
        days as f64,
        days as f64,
        days as f64
    )
    .fetch_all(pool)
    .await?;

    let trend_data: Vec<TrendData> = trends
        .into_iter()
        .map(|row| TrendData {
            date: row.date.unwrap(),
            user_count: row.user_count.unwrap_or(0),
            savings_amount: row.savings_amount.and_then(|d| d.to_string().parse().ok()).unwrap_or(0.0),
            transactions_count: row.transactions_count.unwrap_or(0),
        })
        .collect();

    Ok(trend_data)
}

pub async fn get_rating_distribution(pool: &PgPool) -> Result<Vec<RatingDistribution>> {
    let distributions = sqlx::query!(
        r#"
        SELECT 
            rating,
            COUNT(*) as count,
            (COUNT(*) * 100.0 / (SELECT COUNT(*) FROM testimoni WHERE is_approved = true)) as percentage
        FROM testimoni 
        WHERE is_approved = true
        GROUP BY rating
        ORDER BY rating DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    let rating_dist: Vec<RatingDistribution> = distributions
        .into_iter()
        .map(|row| RatingDistribution {
            rating: row.rating,
            count: row.count.unwrap_or(0),
            percentage: row.percentage.unwrap_or(BigDecimal::from(0)).to_string().parse().unwrap_or(0.0),
        })
        .collect();

    Ok(rating_dist)
}
