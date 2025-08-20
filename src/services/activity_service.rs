use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive};

use crate::models::{
    Activity, CreateActivityRequest, ActivityResponse
};

pub async fn create_activity(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateActivityRequest,
) -> Result<Activity> {
    let amount = BigDecimal::from_f64(req.amount.unwrap_or(0.0))
        .ok_or_else(|| anyhow::anyhow!("Invalid amount"))?;

    let activity = sqlx::query_as!(
        Activity,
        r#"
        INSERT INTO activities (user_id, savings_target_id, activity_type, title, description, amount, icon, icon_color)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, user_id, savings_target_id, activity_type, title, description, 
                  amount, icon, icon_color, created_at
        "#,
        user_id,
        req.savings_target_id,
        req.activity_type,
        req.title,
        req.description,
        amount,
        req.icon.unwrap_or_else(|| "💰".to_string()),
        req.icon_color.unwrap_or_else(|| "bg-blue-500".to_string())
    )
    .fetch_one(pool)
    .await?;

    Ok(activity)
}

pub async fn get_user_activities(
    pool: &PgPool,
    user_id: Uuid,
    limit: Option<i32>,
) -> Result<Vec<ActivityResponse>> {
    let limit = limit.unwrap_or(50);
    
    let activities = sqlx::query_as!(
        Activity,
        r#"
        SELECT id, user_id, savings_target_id, activity_type, title, description,
               amount, icon, icon_color, created_at
        FROM activities 
        WHERE user_id = $1 
        ORDER BY created_at DESC
        LIMIT $2
        "#,
        user_id,
        limit as i64
    )
    .fetch_all(pool)
    .await?;

    let responses: Vec<ActivityResponse> = activities
        .into_iter()
        .map(ActivityResponse::from)
        .collect();

    Ok(responses)
}

pub async fn get_recent_activities_for_dashboard(
    pool: &PgPool,
) -> Result<Vec<ActivityResponse>> {
    let activities = sqlx::query_as!(
        Activity,
        r#"
        SELECT id, user_id, savings_target_id, activity_type, title, description,
               amount, icon, icon_color, created_at
        FROM activities 
        ORDER BY created_at DESC
        LIMIT 20
        "#
    )
    .fetch_all(pool)
    .await?;

    let responses: Vec<ActivityResponse> = activities
        .into_iter()
        .map(ActivityResponse::from)
        .collect();

    Ok(responses)
}

pub async fn create_deposit_activity(
    pool: &PgPool,
    user_id: Uuid,
    savings_target_id: Option<Uuid>,
    amount: f64,
    target_name: Option<String>,
) -> Result<Activity> {
    let decimal_amount = BigDecimal::from_f64(amount)
        .ok_or_else(|| anyhow::anyhow!("Invalid amount"))?;

    let title = match target_name {
        Some(name) => format!("Menabung untuk {}", name),
        None => "Menabung".to_string(),
    };

    let activity = sqlx::query_as!(
        Activity,
        r#"
        INSERT INTO activities (user_id, savings_target_id, activity_type, title, description, amount, icon, icon_color)
        VALUES ($1, $2, 'deposit', $3, $4, $5, '💰', 'bg-green-500')
        RETURNING id, user_id, savings_target_id, activity_type, title, description, 
                  amount, icon, icon_color, created_at
        "#,
        user_id,
        savings_target_id,
        title,
        format!("Setoran sebesar Rp {}", format_currency(amount)),
        decimal_amount
    )
    .fetch_one(pool)
    .await?;

    Ok(activity)
}

pub async fn create_target_created_activity(
    pool: &PgPool,
    user_id: Uuid,
    savings_target_id: Uuid,
    target_name: String,
) -> Result<Activity> {
    let activity = sqlx::query_as!(
        Activity,
        r#"
        INSERT INTO activities (user_id, savings_target_id, activity_type, title, description, amount, icon, icon_color)
        VALUES ($1, $2, 'target_created', $3, $4, 0, '🎯', 'bg-blue-500')
        RETURNING id, user_id, savings_target_id, activity_type, title, description, 
                  amount, icon, icon_color, created_at
        "#,
        user_id,
        savings_target_id,
        "Target baru dibuat",
        format!("Target \"{}\" berhasil dibuat", target_name),
    )
    .fetch_one(pool)
    .await?;

    Ok(activity)
}

pub async fn create_target_completed_activity(
    pool: &PgPool,
    user_id: Uuid,
    savings_target_id: Uuid,
    target_name: String,
) -> Result<Activity> {
    let activity = sqlx::query_as!(
        Activity,
        r#"
        INSERT INTO activities (user_id, savings_target_id, activity_type, title, description, amount, icon, icon_color)
        VALUES ($1, $2, 'target_completed', $3, $4, 0, '🎉', 'bg-green-500')
        RETURNING id, user_id, savings_target_id, activity_type, title, description, 
                  amount, icon, icon_color, created_at
        "#,
        user_id,
        savings_target_id,
        "Target tercapai!",
        format!("Selamat! Target \"{}\" telah tercapai", target_name),
    )
    .fetch_one(pool)
    .await?;

    Ok(activity)
}

fn format_currency(amount: f64) -> String {
    if amount >= 1_000_000.0 {
        format!("{:.1}M", amount / 1_000_000.0)
    } else if amount >= 1_000.0 {
        format!("{:.0}K", amount / 1_000.0)
    } else {
        format!("{:.0}", amount)
    }
}

pub async fn log_withdrawal(
    pool: &PgPool,
    user_id: Uuid,
    target_id: Uuid,
    amount: f64,
) -> Result<Activity> {
    let decimal_amount = BigDecimal::from_f64(amount)
        .ok_or_else(|| anyhow::anyhow!("Invalid amount"))?;

    let activity = sqlx::query_as!(
        Activity,
        r#"
        INSERT INTO activities (user_id, savings_target_id, activity_type, title, description, amount, icon, icon_color)
        VALUES ($1, $2, 'withdrawal', 'Penarikan', $3, $4, '💸', 'bg-red-500')
        RETURNING id, user_id, savings_target_id, activity_type, title, description, 
                  amount, icon, icon_color, created_at
        "#,
        user_id,
        target_id,
        format!("Penarikan sebesar Rp {}", format_currency(amount)),
        decimal_amount
    )
    .fetch_one(pool)
    .await?;

    Ok(activity)
}
