mod config;
mod handlers;
mod models;
mod services;
mod middleware;
mod utils;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware::Logger};
use config::Config;
use sqlx::PgPool;
use std::env;
use actix_web::http::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};

use bcrypt::verify; // tambah ini untuk verify password

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file kalau ada (buat lokal dev)
    if let Err(e) = dotenv::dotenv() {
        eprintln!("⚠️  Warning: .env file tidak ditemukan ({})", e);
        eprintln!("   Railway akan pakai environment variable dari dashboard.");
    }

    env_logger::init();

    // Load konfigurasi (DATABASE_URL, JWT_SECRET, dsb) dari env
    let config = Config::from_env();

    // Railway kasih PORT lewat env → fallback ke 8080 kalau di lokal
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let host = "0.0.0.0";

    // Coba koneksi ke database
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("❌ Gagal connect ke database, cek DATABASE_URL di Railway Variables");

    // Jalankan migrasi SQLx
    if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
        eprintln!("❌ Gagal menjalankan migrasi: {}", e);
        std::process::exit(1);
    }

    // TESTING password verify (manual check)
    test_password_verify();

    println!("🚀 Server starting on {}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // Bisa diganti jadi origin tertentu
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1")
                    .service(handlers::auth::auth_routes())
                    .service(handlers::user::user_routes())
                    .service(handlers::testimoni::testimoni_routes())
                    .service(handlers::dashboard::dashboard_routes())
                    .service(handlers::password::password_routes())
                    .service(handlers::savings::savings_routes())
                    .service(handlers::activity::activity_routes())
                    .service(handlers::statistics::statistics_routes())
                    .service(handlers::reminder::reminder_routes())
                    .configure(handlers::notification::config)
                    .configure(handlers::search::config)
            )
            .service(handlers::health::health_check)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

fn test_password_verify() {
    let plain_password = "admin123";
    let hashed_password = "$2b$12$lsB.1mQxvVlND7dkAQh8vORIMWZxry1gttHtYU4rYGf1fSif2.uRO"; // Contoh hash dari DB

    match verify(plain_password, hashed_password) {
        Ok(valid) => println!("Password valid? {}", valid),
        Err(e) => println!("Error verifying password: {}", e),
    }
}
