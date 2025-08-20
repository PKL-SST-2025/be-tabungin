use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use bigdecimal::{BigDecimal, FromPrimitive};

use crate::models::{UserStatistics, Achievement};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStatisticsResponse {
    pub total_saved: f64,
    pub streak_days: i32,
    pub daily_average: f64,
    pub achievements_count: i32,
    pub last_deposit_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchievementResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub icon_color: String,
    pub earned_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreakDayData {
    pub date: chrono::NaiveDate,
    pub has_deposit: bool,
    pub deposit_amount: Option<f64>,
    pub is_today: bool,
    pub is_part_of_streak: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreakDataResponse {
    pub current_streak: i32,
    pub days: Vec<StreakDayData>,
}

pub async fn get_user_statistics(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<UserStatisticsResponse> {
    let stats = sqlx::query_as!(
        UserStatistics,
        r#"
        SELECT id, user_id, total_saved, streak_days, daily_average, achievements_count, 
               last_deposit_date, created_at, updated_at
        FROM user_statistics 
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    let stats = match stats {
        Some(s) => s,
        None => {
            // Create default statistics if none exist
            create_default_user_statistics(pool, user_id).await?
        }
    };

    Ok(UserStatisticsResponse {
        total_saved: stats.total_saved.as_ref().map(|v| v.to_string().parse().unwrap_or(0.0)).unwrap_or(0.0),
        streak_days: stats.streak_days.unwrap_or(0),
        daily_average: stats.daily_average.as_ref().map(|v| v.to_string().parse().unwrap_or(0.0)).unwrap_or(0.0),
        achievements_count: stats.achievements_count.unwrap_or(0),
        last_deposit_date: stats.last_deposit_date,
    })
}

pub async fn get_user_achievements(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<AchievementResponse>> {
    let achievements = sqlx::query_as!(
        Achievement,
        r#"
        SELECT id, user_id, title, description, icon, icon_color, earned_at
        FROM achievements 
        WHERE user_id = $1 
        ORDER BY earned_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    let responses: Vec<AchievementResponse> = achievements
        .into_iter()
        .map(|a| AchievementResponse {
            id: a.id,
            title: a.title,
            description: a.description,
            icon: a.icon,
            icon_color: a.icon_color,
            earned_at: a.earned_at.unwrap_or_else(|| chrono::Utc::now()),
        })
        .collect();

    Ok(responses)
}

pub async fn get_user_streak_data(
    pool: &PgPool,
    user_id: Uuid,
    days: i32,
) -> Result<StreakDataResponse> {
    // Get current streak from database (for reference)
    let _stats = get_user_statistics(pool, user_id).await?;
    
    // Get deposit activities for the specified number of days FROM TODAY
    // Always use current date as reference point
    let deposit_dates = sqlx::query!(
        r#"
        SELECT DATE(created_at) as deposit_date, SUM(amount) as total_amount
        FROM activities 
        WHERE user_id = $1 
        AND activity_type = 'deposit'
        AND DATE(created_at) >= CURRENT_DATE - INTERVAL '1 day' * ($2 - 1)
        AND DATE(created_at) <= CURRENT_DATE
        GROUP BY DATE(created_at)
        ORDER BY deposit_date ASC
        "#,
        user_id,
        days as i64
    )
    .fetch_all(pool)
    .await?;

    println!("Deposit dates from DB: {:?}", deposit_dates); // Debug log

    // Create deposit map for quick lookup
    let mut deposit_map = std::collections::HashMap::new();
    for record in deposit_dates {
        if let (Some(date), Some(amount)) = (record.deposit_date, record.total_amount) {
            let amount_f64 = amount.to_string().parse::<f64>().unwrap_or(0.0);
            deposit_map.insert(date, amount_f64);
        }
    }

    println!("Deposit map: {:?}", deposit_map); // Debug log

    // Calculate actual consecutive streak from today backwards
    // Force use current date in Indonesia timezone (UTC+7)
    let today = chrono::Utc::now()
        .with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).unwrap())
        .date_naive();
    println!("Today in backend: {}", today); // Debug log
    let mut actual_streak = 0;
    let mut check_date = today;
    
    // Count consecutive days with deposits from today backwards
    loop {
        if deposit_map.contains_key(&check_date) {
            actual_streak += 1;
            check_date = check_date - chrono::Duration::days(1);
        } else {
            break;
        }
    }

    // Generate days data - show dates in chronological order
    let mut days_data = Vec::new();

    // Start from the earliest date we want to show
    let start_date = today - chrono::Duration::days((days - 1) as i64);
    
    for i in 0..days {
        let date = start_date + chrono::Duration::days(i as i64);
        let has_deposit = deposit_map.contains_key(&date);
        let deposit_amount = deposit_map.get(&date).copied();
        let is_today = date == today;
        
        // A day is part of streak if:
        // 1. It has a deposit AND
        // 2. It's within the consecutive streak period from today backwards
        let days_from_today = (today - date).num_days();
        let is_part_of_streak = has_deposit && days_from_today < actual_streak as i64;

        days_data.push(StreakDayData {
            date,
            has_deposit,
            deposit_amount,
            is_today,
            is_part_of_streak,
        });
    }

    Ok(StreakDataResponse {
        current_streak: actual_streak, // Use the calculated streak
        days: days_data,
    })
}

pub async fn update_user_statistics_after_deposit(
    pool: &PgPool,
    user_id: Uuid,
    deposit_amount: f64,
) -> Result<()> {
    let decimal_amount = BigDecimal::from_f64(deposit_amount)
        .ok_or_else(|| anyhow::anyhow!("Invalid deposit amount"))?;

    // Update statistics
    sqlx::query!(
        r#"
        INSERT INTO user_statistics (user_id, total_saved, streak_days, daily_average, achievements_count, last_deposit_date)
        VALUES ($1, $2, 1, $2, 0, CURRENT_DATE)
        ON CONFLICT (user_id) DO UPDATE SET
            total_saved = user_statistics.total_saved + $2,
            last_deposit_date = CURRENT_DATE,
            streak_days = CASE 
                WHEN user_statistics.last_deposit_date = CURRENT_DATE - INTERVAL '1 day' 
                THEN user_statistics.streak_days + 1
                WHEN user_statistics.last_deposit_date = CURRENT_DATE 
                THEN user_statistics.streak_days
                ELSE 1
            END,
            daily_average = (user_statistics.total_saved + $2) / GREATEST(
                (CURRENT_DATE - (SELECT created_at::date FROM users WHERE id = $1))::integer,
                1
            ),
            updated_at = NOW()
        "#,
        user_id,
        decimal_amount
    )
    .execute(pool)
    .await?;

    // Check for new achievements
    check_and_award_achievements(pool, user_id).await?;

    Ok(())
}

async fn create_default_user_statistics(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<UserStatistics> {
    let stats = sqlx::query_as!(
        UserStatistics,
        r#"
        INSERT INTO user_statistics (user_id, total_saved, streak_days, daily_average, achievements_count, last_deposit_date)
        VALUES ($1, 0, 0, 0, 0, NULL)
        RETURNING id, user_id, total_saved, streak_days, daily_average, achievements_count, 
                  last_deposit_date, created_at, updated_at
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(stats)
}

async fn check_and_award_achievements(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<()> {
    let stats = get_user_statistics(pool, user_id).await?;

    // Check for streak achievements
    if stats.streak_days >= 10 {
        award_achievement_if_not_exists(
            pool,
            user_id,
            "Streak 10 Hari!".to_string(),
            "Konsisten menabung 10 hari berturut-turut".to_string(),
            "🏆".to_string(),
            "bg-yellow-500".to_string(),
        ).await?;
    }

    // Check for total saved achievements
    if stats.total_saved >= 10_000_000.0 {
        award_achievement_if_not_exists(
            pool,
            user_id,
            "RP 10M+".to_string(),
            "Total tabungan yang terkumpul mencapai 10M+".to_string(),
            "💰".to_string(),
            "bg-green-500".to_string(),
        ).await?;
    }

    // Check for completed targets achievement
    let completed_targets = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM savings_targets WHERE user_id = $1 AND is_completed = true",
        user_id
    )
    .fetch_one(pool)
    .await?;

    if completed_targets.unwrap_or(0) >= 3 {
        award_achievement_if_not_exists(
            pool,
            user_id,
            "Target Master".to_string(),
            "Berhasil mencapai 3 target tabungan".to_string(),
            "🎯".to_string(),
            "bg-red-500".to_string(),
        ).await?;
    }

    Ok(())
}

async fn award_achievement_if_not_exists(
    pool: &PgPool,
    user_id: Uuid,
    title: String,
    description: String,
    icon: String,
    icon_color: String,
) -> Result<()> {
    // Check if achievement already exists
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM achievements WHERE user_id = $1 AND title = $2)",
        user_id,
        title
    )
    .fetch_one(pool)
    .await?;

    if !exists.unwrap_or(false) {
        sqlx::query!(
            r#"
            INSERT INTO achievements (user_id, title, description, icon, icon_color)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            user_id,
            title,
            description,
            icon,
            icon_color
        )
        .execute(pool)
        .await?;

        // Update achievements count
        sqlx::query!(
            r#"
            UPDATE user_statistics 
            SET achievements_count = achievements_count + 1
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}
