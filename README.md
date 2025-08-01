# Tabungin API

Backend API untuk aplikasi Tabungin menggunakan Rust dan Actix Web.

## Features

- Authentication (Register/Login)
- User Management
- Testimoni Management
- Dashboard Analytics
- JWT-based Authorization
- PostgreSQL Database
- CORS Support

## Prerequisites

- Rust (latest stable)
- PostgreSQL
- sqlx-cli

## Setup

1. Install sqlx-cli:
```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

2. Setup environment variables:
```bash
cp .env.example .env
# Edit .env with your database credentials
```

3. Setup database:
```bash
# Create database
createdb tabungin_db

# Run migrations
sqlx migrate run
```

4. Run the application:
```bash
cargo run
```

The API will be available at `http://localhost:8080`

## API Endpoints

### Authentication
- `POST /api/v1/auth/register` - Register new user
- `POST /api/v1/auth/login` - Login user

### Users
- `GET /api/v1/users/profile` - Get user profile (authenticated)
- `PUT /api/v1/users/profile` - Update user profile (authenticated)
- `GET /api/v1/users` - Get all users (admin only)

### Testimoni
- `POST /api/v1/testimoni` - Create testimoni (authenticated)
- `GET /api/v1/testimoni/approved` - Get approved testimoni (public)
- `GET /api/v1/testimoni/all` - Get all testimoni (admin only)
- `GET /api/v1/testimoni/my` - Get user's testimoni (authenticated)
- `PUT /api/v1/testimoni/{id}` - Update testimoni (authenticated)
- `DELETE /api/v1/testimoni/{id}` - Delete testimoni (authenticated)

### Dashboard
- `GET /api/v1/dashboard/stats` - Get dashboard statistics (admin only)
- `GET /api/v1/dashboard/analytics` - Get user analytics (authenticated)

### Health Check
- `GET /health` - Health check endpoint

## Default Admin User

Email: `admin@tabungin.com`
Password: `admin123`

## Environment Variables

```
DATABASE_URL=postgres://username:password@localhost/tabungin_db
JWT_SECRET=your_super_secret_jwt_key_here_change_in_production
RUST_LOG=debug
HOST=127.0.0.1
PORT=8080
```

## Development

1. Run in development mode:
```bash
cargo run
```

2. Run tests:
```bash
cargo test
```

3. Format code:
```bash
cargo fmt
```

4. Run clippy:
```bash
cargo clippy
```
