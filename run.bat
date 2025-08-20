@echo off
echo 🚀 Starting Tabungin API...
echo.

REM Check if .env exists
if not exist .env (
    echo ❌ .env file not found!
    echo Please copy .env.example to .env and configure it
    exit /b 1
)

REM Load environment and run
echo 📡 Loading environment variables...
echo 🗄️  Connecting to database...
echo 🌐 Starting server...
echo.

cargo run

echo.
echo ⚠️  Server stopped.
pause
