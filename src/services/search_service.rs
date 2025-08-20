use sqlx::PgPool;

pub async fn search_users(pool: &PgPool, query: &str) -> Result<Vec<(i64, String, String)>, sqlx::Error> {
    let like_query = format!("%{}%", query);
    let rows = sqlx::query!(
        "SELECT id, full_name, email FROM users WHERE full_name ILIKE $1 OR email ILIKE $1",
        like_query
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|r| (r.id, r.full_name, r.email)).collect())
}
// Tambahkan fungsi search lain sesuai kebutuhan (transaksi, dsb)
