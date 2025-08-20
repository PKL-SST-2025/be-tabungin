use sqlx::PgPool;
use bcrypt::{hash, DEFAULT_COST};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Get database URL
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/tabungin_db".to_string());
    
    // Connect to database
    let pool = PgPool::connect(&database_url).await?;
    
    println!("🔧 Updating passwords to bcrypt hash...");
    
    // Update admin password
    let admin_hashed = hash("admin123", DEFAULT_COST)?;
    sqlx::query("UPDATE users SET password_hash = $1 WHERE email = 'admin@tabungin.com'")
        .bind(admin_hashed)
        .execute(&pool)
        .await?;
    println!("✅ Updated admin@tabungin.com password");
    
    // Check if umar@app.com exists and update
    let user_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE email = 'umar@app.com'")
        .fetch_one(&pool)
        .await?;
    
    if user_count.0 > 0 {
        let umar_hashed = hash("password123", DEFAULT_COST)?;
        sqlx::query("UPDATE users SET password_hash = $1 WHERE email = 'umar@app.com'")
            .bind(umar_hashed)
            .execute(&pool)
            .await?;
        println!("✅ Updated umar@app.com password");
    } else {
        // Create umar user
        let umar_hashed = hash("password123", DEFAULT_COST)?;
        sqlx::query(
            r#"
            INSERT INTO users (id, full_name, email, password_hash, is_admin, created_at, updated_at)
            VALUES (gen_random_uuid(), 'Umar Said', 'umar@app.com', $1, false, NOW(), NOW())
            "#
        )
        .bind(umar_hashed)
        .execute(&pool)
        .await?;
        println!("✅ Created umar@app.com user with password");
    }
    
    println!("🎉 Password update completed!");
    
    Ok(())
}
