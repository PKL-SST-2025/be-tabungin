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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = Config::from_env();
    let host = config.host.clone();
    let port = config.port;

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
            )
            .service(handlers::health::health_check)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
