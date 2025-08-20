use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;
use bigdecimal::BigDecimal;
use crate::models::{SavingsTarget, CreateSavingsTargetRequest, UpdateSavingsTargetRequest, SavingsTargetResponse};

pub async fn create_savings_target(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateSavingsTargetRequest,
) -> Result<SavingsTarget> {
    let decimal_amount = BigDecimal::try_from(req.target_amount)
        .map_err(|_| anyhow::anyhow!("Invalid target amount"))?;

    let savings_target = sqlx::query_as!(
        SavingsTarget,
        r#"
        INSERT INTO savings_targets (user_id, name, target_amount, icon, icon_color, target_date)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, user_id, name, target_amount, current_amount, icon, icon_color,
                  target_date, is_completed, created_at, updated_at
        "#,
        user_id,
        req.name,
        decimal_amount,
        req.icon.unwrap_or_else(|| "💰".to_string()),
        req.icon_color.unwrap_or_else(|| "bg-blue-500".to_string()),
        req.target_date
    )
    .fetch_one(pool)
    .await?;

    // Activity creation is now handled in the handler to avoid duplication
    // Only create activity here to be sure it's done once
    let _ = crate::services::activity_service::create_target_created_activity(
        pool,
        user_id,
        savings_target.id,
        savings_target.name.clone(),
    ).await;

    Ok(savings_target)
}

pub async fn get_user_savings_targets(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<SavingsTarget>> {
    let targets = sqlx::query_as!(
        SavingsTarget,
        r#"
        SELECT id, user_id, name, target_amount, current_amount, icon, icon_color,
               target_date, is_completed, created_at, updated_at
        FROM savings_targets
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(targets)
}

pub async fn get_savings_target_by_id(
    pool: &PgPool,
    target_id: Uuid,
    user_id: Uuid,
) -> Result<Option<SavingsTarget>> {
    let target = sqlx::query_as!(
        SavingsTarget,
        r#"
        SELECT id, user_id, name, target_amount, current_amount, icon, icon_color,
               target_date, is_completed, created_at, updated_at
        FROM savings_targets
        WHERE id = $1 AND user_id = $2
        "#,
        target_id,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(target)
}

pub async fn add_deposit_to_target(
    pool: &PgPool,
    user_id: Uuid,
    target_id: Uuid,
    amount: f64,
) -> Result<SavingsTarget> {
    let decimal_amount = BigDecimal::try_from(amount)
        .map_err(|_| anyhow::anyhow!("Invalid deposit amount"))?;

    let mut tx = pool.begin().await?;

    // First check if target exists and belongs to user
    let existing_target = sqlx::query!(
        "SELECT id, user_id, name, current_amount, target_amount FROM savings_targets WHERE id = $1",
        target_id
    )
    .fetch_optional(&mut *tx)
    .await?;

    match existing_target {
        None => {
            return Err(anyhow::anyhow!("Savings target with ID {} not found", target_id));
        }
        Some(target) => {
            if target.user_id != user_id {
                return Err(anyhow::anyhow!(
                    "Access denied: target belongs to user {} but requested by user {}", 
                    target.user_id, user_id
                ));
            }
            println!("Target found: {} (Current: {:?}, Adding: {})", target.name, target.current_amount, amount);
        }
    }

    // Update target
    let target = sqlx::query_as!(
        SavingsTarget,
        r#"
        UPDATE savings_targets
        SET current_amount = current_amount + $1,
            is_completed = CASE 
                WHEN current_amount + $1 >= target_amount THEN true
                ELSE is_completed
            END,
            updated_at = NOW()
        WHERE id = $2 AND user_id = $3
        RETURNING id, user_id, name, target_amount, current_amount, icon, icon_color,
                  target_date, is_completed, created_at, updated_at
        "#,
        decimal_amount,
        target_id,
        user_id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| anyhow::anyhow!("Failed to update target: {}", e))?;

    tx.commit().await?;

    // After successful transaction, create activity and update statistics
    let _ = crate::services::activity_service::create_deposit_activity(
        pool, 
        user_id, 
        Some(target_id), 
        amount, 
        Some(target.name.clone())
    ).await;

    // Check if target is completed and log achievement
    if target.is_completed.unwrap_or(false) {
        let _ = crate::services::activity_service::create_target_completed_activity(
            pool,
            user_id,
            target_id,
            target.name.clone(),
        ).await;
    }

    // Update user statistics
    let _ = crate::services::statistics_service::update_user_statistics_after_deposit(
        pool, 
        user_id, 
        amount
    ).await;

    Ok(target)
}

pub async fn withdraw_from_target(
    pool: &PgPool,
    target_id: Uuid,
    user_id: Uuid,
    amount: f64,
) -> Result<SavingsTarget> {
    let decimal_amount = BigDecimal::try_from(amount)
        .map_err(|_| anyhow::anyhow!("Invalid withdrawal amount"))?;

    let target = sqlx::query_as!(
        SavingsTarget,
        r#"
        UPDATE savings_targets
        SET current_amount = GREATEST(current_amount - $1, 0),
            is_completed = CASE 
                WHEN current_amount - $1 < target_amount THEN false
                ELSE is_completed
            END,
            updated_at = NOW()
        WHERE id = $2 AND user_id = $3
        RETURNING id, user_id, name, target_amount, current_amount, icon, icon_color,
                  target_date, is_completed, created_at, updated_at
        "#,
        decimal_amount,
        target_id,
        user_id
    )
    .fetch_one(pool)
    .await?;

    // Log activity
    crate::services::activity_service::log_withdrawal(pool, user_id, target_id, amount).await?;

    Ok(target)
}

pub async fn delete_savings_target(
    pool: &PgPool,
    target_id: Uuid,
    user_id: Uuid,
) -> Result<bool> {
    let result = sqlx::query!(
        "DELETE FROM savings_targets WHERE id = $1 AND user_id = $2",
        target_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn update_savings_target(
    pool: &PgPool,
    target_id: Uuid,
    user_id: Uuid,
    req: UpdateSavingsTargetRequest,
) -> Result<Option<SavingsTarget>> {
    let target = sqlx::query_as!(
        SavingsTarget,
        r#"
        UPDATE savings_targets
        SET name = COALESCE($1, name),
            target_amount = COALESCE($2, target_amount),
            current_amount = COALESCE($3, current_amount),
            icon = COALESCE($4, icon),
            icon_color = COALESCE($5, icon_color),
            target_date = COALESCE($6, target_date),
            is_completed = COALESCE($7, is_completed),
            updated_at = NOW()
        WHERE id = $8 AND user_id = $9
        RETURNING id, user_id, name, target_amount, current_amount, icon, icon_color,
                  target_date, is_completed, created_at, updated_at
        "#,
        req.name,
        req.target_amount.map(BigDecimal::try_from).transpose().map_err(|_| anyhow::anyhow!("Invalid amount"))?,
        req.current_amount.map(BigDecimal::try_from).transpose().map_err(|_| anyhow::anyhow!("Invalid amount"))?,
        req.icon,
        req.icon_color,
        req.target_date,
        req.is_completed,
        target_id,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(target)
}
