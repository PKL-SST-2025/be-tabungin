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
use std::net::SocketAddr;
use actix_web::http::{header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}, Method};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file dari current directory
    if let Err(e) = dotenv::dotenv() {
        eprintln!("Warning: Could not load .env file: {}", e);
        eprintln!("Make sure .env file exists in the project root");
        eprintln!("Using default environment variables...");
        
        // Set default environment variables jika .env gagal dimuat
        env::set_var("DATABASE_URL", "postgresql://postgres:password@localhost:5432/tabungin_db");
        env::set_var("JWT_SECRET", "your-super-secret-jwt-key-here");
        env::set_var("HOST", "127.0.0.1");
        env::set_var("PORT", "8080");
        env::set_var("RUST_LOG", "debug");

        // CORS configuration untuk development
    let local_origin = "http://localhost:5173".parse::<HeaderValue>().unwrap();
    let vercel_origin = "https://fe-tabungin.vercel.app/".parse::<HeaderValue>().unwrap();
    let cors = CorsLayer::new()
        .allow_origin([local_origin, vercel_origin])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT])
        .allow_credentials(true);
    }
    
    env_logger::init();

    let config = Config::from_env();

    // Railway memberikan PORT lewat environment variable, fallback ke 8080 kalau lokal
    let port: u16 = env::var("PORT")    
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    // Untuk bind ke semua interface supaya bisa diakses dari luar container
    let host = "0.0.0.0";

    // Database connection
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    println!("🚀 Server starting on {}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
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
