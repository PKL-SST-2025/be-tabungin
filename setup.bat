@echo off
echo 🚀 Setting up Tabungin API...

REM Check if Rust is installed
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Rust is not installed. Please install Rust first.
    echo Visit: https://rustup.rs/
    exit /b 1
)

REM Install sqlx-cli if not already installed
sqlx --version >nul 2>&1
if %errorlevel% neq 0 (
    echo 📦 Installing sqlx-cli...
    cargo install sqlx-cli --no-default-features --features rustls,postgres
)

REM Copy environment file if it doesn't exist
if not exist .env (
    echo 📋 Creating .env file...
    copy .env.example .env
    echo ⚠️  Please edit .env file with your database credentials
)

echo 🗄️  Please ensure PostgreSQL is running and create the database manually:
echo    createdb tabungin_db

echo 🔄 To run database migrations, use:
echo    sqlx migrate run

echo ✅ Setup files created!
echo.
echo 🚀 To start the server, run:
echo    cargo run
echo.
echo 📚 API will be available at: http://localhost:8080
echo 🩺 Health check: http://localhost:8080/health
