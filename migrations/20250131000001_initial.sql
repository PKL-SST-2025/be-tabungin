-- Add migration script here

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    full_name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    avatar TEXT,
    is_admin BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create testimoni table
CREATE TABLE IF NOT EXISTS testimoni (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    is_approved BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at);
CREATE INDEX IF NOT EXISTS idx_testimoni_user_id ON testimoni(user_id);
CREATE INDEX IF NOT EXISTS idx_testimoni_is_approved ON testimoni(is_approved);
CREATE INDEX IF NOT EXISTS idx_testimoni_created_at ON testimoni(created_at);

-- Insert default admin user (password: admin123)
INSERT INTO users (id, full_name, email, password_hash, is_admin, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    'Admin User',
    'admin@tabungin.com',
    'admin123', -- plain password for dev
    TRUE,
    NOW(),
    NOW()
) ON CONFLICT (email) DO NOTHING;
