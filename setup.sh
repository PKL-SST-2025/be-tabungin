#!/bin/bash

# Development setup script for Tabungin API

echo "🚀 Setting up Tabungin API..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first."
    echo "Visit: https://rustup.rs/"
    exit 1
fi

# Check if PostgreSQL is running
if ! pg_isready &> /dev/null; then
    echo "❌ PostgreSQL is not running. Please start PostgreSQL first."
    exit 1
fi

# Install sqlx-cli if not already installed
if ! command -v sqlx &> /dev/null; then
    echo "📦 Installing sqlx-cli..."
    cargo install sqlx-cli --no-default-features --features rustls,postgres
fi

# Copy environment file if it doesn't exist
if [ ! -f .env ]; then
    echo "📋 Creating .env file..."
    cp .env.example .env
    echo "⚠️  Please edit .env file with your database credentials"
fi

# Source the environment file
source .env

# Create database if it doesn't exist
echo "🗄️  Setting up database..."
createdb $DATABASE_NAME 2>/dev/null || echo "Database already exists"

# Run migrations
echo "🔄 Running database migrations..."
sqlx migrate run

echo "✅ Setup complete!"
echo ""
echo "🚀 To start the server, run:"
echo "   cargo run"
echo ""
echo "📚 API will be available at: http://localhost:8080"
echo "🩺 Health check: http://localhost:8080/health"
