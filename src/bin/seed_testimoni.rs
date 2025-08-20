use sqlx::PgPool;
use uuid::Uuid;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Get database URL
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    // Connect to database
    let pool = PgPool::connect(&database_url).await?;
    
    println!("🌱 Starting testimoni seeding...");
    
    // Sample testimoni data
    let testimoni_data = vec![
        (
            "Aplikasi Tabungin sangat membantu dalam financial planning! Target tabungan untuk wedding berhasil tercapai dalam 8 bulan. Fitur automatic saving dan budget tracking-nya luar biasa. Highly recommended! 💍",
            5,
            true,
            7
        ),
        (
            "Interface yang user-friendly dan fitur-fitur yang lengkap. Sebagai working mom, saya terbantu dengan fitur family budgeting. Anak-anak juga jadi belajar menabung dari aplikasi ini. Great job team! 👨‍👩‍👧‍👦",
            5,
            true,
            14
        ),
        (
            "Dulu saya tipe yang boros dan impulsive buying. Setelah pakai Tabungin, spending habits jadi lebih terkontrol. Fitur expense categorization sangat membantu analisis pengeluaran bulanan.",
            4,
            true,
            21
        ),
        (
            "Aplikasi bagus untuk pemula yang ingin belajar financial literacy. Dashboard analytics cukup informatif. Mungkin bisa ditambah fitur investment planning untuk yang sudah advanced.",
            4,
            true,
            10
        ),
        (
            "Fitur goal-based saving sangat motivational! Progress tracking yang visual membuat semangat terus menabung. Customer support juga responsif ketika ada technical issue.",
            5,
            true,
            5
        ),
        (
            "Sebagai freelancer dengan income fluktuatif, Tabungin membantu mengatur cash flow dengan lebih baik. Fitur flexible budgeting sesuai dengan nature pekerjaan freelance.",
            4,
            true,
            18
        ),
        (
            "Security system yang reliable dan user experience yang smooth. Multi-device sync juga bekerja dengan baik. Overall satisfied dengan performance aplikasi ini.",
            4,
            true,
            12
        ),
        (
            "Aplikasi cukup bagus untuk basic saving activities. Loading speed kadang agak lambat dan beberapa fitur advanced masih perlu improvement. Tapi untuk harga segini worth it lah.",
            3,
            true,
            25
        ),
        (
            "Game-changer untuk financial management! Dari yang tadinya selalu minus di akhir bulan, sekarang bisa saving 25% dari salary. Achievement system-nya juga fun dan engaging! 🏆",
            5,
            false,
            2
        ),
        (
            "Baru pakai 2 minggu tapi sudah keliatan hasilnya. Spending awareness jadi meningkat drastis. Notification reminder juga membantu maintain saving discipline. Keep up the good work!",
            5,
            false,
            1
        ),
        (
            "Integration dengan mobile banking akan lebih praktis. Dark mode juga masih belum ada. Tapi untuk core functionality sudah sangat membantu organizing finances.",
            3,
            false,
            3
        ),
        (
            "Excellent app for financial education! Artikel-artikel di education center sangat bermanfaat. Features lengkap dan execution-nya solid. Definitely recommend to everyone! 📚",
            5,
            false,
            4
        ),
    ];
    
    // Get existing users (fallback to admin if no other users)
    let users: Vec<Uuid> = sqlx::query_scalar("SELECT id FROM users LIMIT 10")
        .fetch_all(&pool)
        .await?;
    
    if users.is_empty() {
        eprintln!("❌ No users found in database. Please run user migrations first.");
        return Ok(());
    }
    
    println!("👥 Found {} users in database", users.len());
    
    // Insert testimoni data
    let mut inserted_count = 0;
    for (i, (content, rating, is_approved, days_ago)) in testimoni_data.iter().enumerate() {
        let user_id = users[i % users.len()]; // Distribute testimoni across available users
        
        let result = sqlx::query!(
            r#"
            INSERT INTO testimoni (id, user_id, content, rating, is_approved, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW() - INTERVAL '1 day' * $6, NOW() - INTERVAL '1 day' * $6)
            ON CONFLICT (id) DO NOTHING
            "#,
            Uuid::new_v4(),
            user_id,
            content,
            rating,
            is_approved,
            *days_ago as i32
        )
        .execute(&pool)
        .await;
        
        match result {
            Ok(_) => {
                inserted_count += 1;
                println!("✅ Inserted testimoni {} (rating: {}, approved: {})", inserted_count, rating, is_approved);
            }
            Err(e) => {
                eprintln!("❌ Failed to insert testimoni {}: {}", i + 1, e);
            }
        }
    }
    
    // Generate summary
    let summary = sqlx::query!(
        r#"
        SELECT 
            rating,
            is_approved,
            COUNT(*) as count
        FROM testimoni
        GROUP BY rating, is_approved
        ORDER BY rating DESC, is_approved DESC
        "#
    )
    .fetch_all(&pool)
    .await?;
    
    println!("\n📊 Testimoni Summary:");
    println!("┌─────────┬──────────┬───────┐");
    println!("│ Rating  │ Approved │ Count │");
    println!("├─────────┼──────────┼───────┤");
    for row in summary {
        println!("│   {}     │    {}     │   {}   │", 
                 row.rating, 
                 if row.is_approved.unwrap_or(false) { "✓" } else { "✗" }, 
                 row.count.unwrap_or(0));
    }
    println!("└─────────┴──────────┴───────┘");
    
    println!("\n🎉 Testimoni seeding completed! Inserted {} new testimoni.", inserted_count);
    
    Ok(())
}
