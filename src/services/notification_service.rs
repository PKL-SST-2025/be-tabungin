use crate::models::Notification;
use sqlx::PgPool;

pub async fn get_notifications(pool: &PgPool, user_id: uuid::Uuid) -> Result<Vec<Notification>, sqlx::Error> {
    let rows = sqlx::query_as!(Notification,
        r#"SELECT id, user_id, type, message, read, timestamp FROM notifications WHERE user_id = $1 ORDER BY timestamp DESC"#,
        user_id
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn add_notification(pool: &PgPool, notif: &Notification) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO notifications (user_id, type, message, read, timestamp) VALUES ($1, $2, $3, $4, $5)",
        notif.user_id,
        notif.r#type,
        notif.message,
        notif.read,
        notif.timestamp
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn mark_as_read(pool: &PgPool, notif_id: uuid::Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE notifications SET read = true WHERE id = $1",
        notif_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
