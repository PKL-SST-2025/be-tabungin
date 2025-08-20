use crate::models::ReminderResponse;
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

pub struct ReminderService {
    pool: PgPool,
}

impl ReminderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_user_reminders(&self, user_id: Uuid, limit: Option<i32>) -> Result<Vec<ReminderResponse>> {
        let limit = limit.unwrap_or(50);
        
        let reminders = sqlx::query_as!(
            ReminderResponse,
            r#"
            SELECT 
                r.id,
                r.reminder_date,
                r.reminder_type,
                r.title,
                r.description as "description!",
                r.is_completed as "is_completed!",
                r.is_notified as "is_notified!",
                st.name as target_name,
                st.icon as "target_icon!",
                st.icon_color as "target_icon_color!",
                r.created_at as "created_at!"
            FROM reminders r
            JOIN savings_targets st ON r.savings_target_id = st.id
            WHERE r.user_id = $1
            ORDER BY r.reminder_date ASC, r.created_at DESC
            LIMIT $2
            "#,
            user_id,
            limit as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(reminders)
    }

    pub async fn get_upcoming_reminders(&self, user_id: Uuid, days: Option<i32>) -> Result<Vec<ReminderResponse>> {
        let days = days.unwrap_or(30) as f64;
        
        let reminders = sqlx::query_as!(
            ReminderResponse,
            r#"
            SELECT 
                r.id,
                r.reminder_date,
                r.reminder_type,
                r.title,
                r.description as "description!",
                r.is_completed as "is_completed!",
                r.is_notified as "is_notified!",
                st.name as target_name,
                st.icon as "target_icon!",
                st.icon_color as "target_icon_color!",
                r.created_at as "created_at!"
            FROM reminders r
            JOIN savings_targets st ON r.savings_target_id = st.id
            WHERE r.user_id = $1 
              AND r.reminder_date >= CURRENT_DATE 
              AND r.reminder_date <= CURRENT_DATE + $2 * INTERVAL '1 day'
              AND r.is_completed = false
            ORDER BY r.reminder_date ASC, r.created_at DESC
            "#,
            user_id,
            days
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(reminders)
    }

    pub async fn mark_reminder_completed(&self, reminder_id: Uuid, user_id: Uuid) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            UPDATE reminders 
            SET is_completed = true, updated_at = NOW()
            WHERE id = $1 AND user_id = $2
            "#,
            reminder_id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn mark_reminder_notified(&self, reminder_id: Uuid) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            UPDATE reminders 
            SET is_notified = true, updated_at = NOW()
            WHERE id = $1
            "#,
            reminder_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_todays_reminders(&self, user_id: Uuid) -> Result<Vec<ReminderResponse>> {
        let reminders = sqlx::query_as!(
            ReminderResponse,
            r#"
            SELECT 
                r.id,
                r.reminder_date,
                r.reminder_type,
                r.title,
                r.description as "description!",
                r.is_completed as "is_completed!",
                r.is_notified as "is_notified!",
                st.name as target_name,
                st.icon as "target_icon!",
                st.icon_color as "target_icon_color!",
                r.created_at as "created_at!"
            FROM reminders r
            JOIN savings_targets st ON r.savings_target_id = st.id
            WHERE r.user_id = $1 
              AND r.reminder_date = CURRENT_DATE
              AND r.is_completed = false
            ORDER BY r.created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(reminders)
    }

    pub async fn get_calendar_events(&self, user_id: Uuid, month: i32, year: i32) -> Result<Vec<ReminderResponse>> {
        let start_date = chrono::NaiveDate::from_ymd_opt(year, month as u32, 1)
            .ok_or_else(|| anyhow::anyhow!("Invalid date"))?;
        let end_date = if month == 12 {
            chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            chrono::NaiveDate::from_ymd_opt(year, month as u32 + 1, 1)
        }.ok_or_else(|| anyhow::anyhow!("Invalid date"))?;

        println!("🗓️ Calendar events query - User: {}, Month: {}, Year: {}", user_id, month, year);
        println!("🗓️ Date range: {} to {}", start_date, end_date);

        let reminders = sqlx::query_as!(
            ReminderResponse,
            r#"
            SELECT 
                r.id,
                r.reminder_date,
                r.reminder_type,
                r.title,
                r.description as "description!",
                r.is_completed as "is_completed!",
                r.is_notified as "is_notified!",
                st.name as target_name,
                st.icon as "target_icon!",
                st.icon_color as "target_icon_color!",
                r.created_at as "created_at!"
            FROM reminders r
            JOIN savings_targets st ON r.savings_target_id = st.id
            WHERE r.user_id = $1 
              AND r.reminder_date >= $2
              AND r.reminder_date < $3
            ORDER BY r.reminder_date ASC
            "#,
            user_id,
            start_date,
            end_date
        )
        .fetch_all(&self.pool)
        .await?;

        println!("🗓️ Found {} calendar events", reminders.len());
        for reminder in &reminders {
            println!("📅 Event: {} on {} (completed: {})", reminder.title, reminder.reminder_date, reminder.is_completed);
        }

        Ok(reminders)
    }
}
