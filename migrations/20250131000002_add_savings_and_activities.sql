-- Add savings targets and activities tables

-- Create savings_targets table
CREATE TABLE IF NOT EXISTS savings_targets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    target_amount DECIMAL(15,2) NOT NULL,
    current_amount DECIMAL(15,2) DEFAULT 0.00,
    icon VARCHAR(50) DEFAULT '📌',
    icon_color VARCHAR(50) DEFAULT 'bg-purple-500',
    target_date DATE,
    is_completed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create activities table
CREATE TABLE IF NOT EXISTS activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    savings_target_id UUID REFERENCES savings_targets(id) ON DELETE SET NULL,
    activity_type VARCHAR(50) NOT NULL, -- 'deposit', 'withdraw', 'target_created', 'target_completed', 'achievement'
    title VARCHAR(255) NOT NULL,
    description TEXT,
    amount DECIMAL(15,2) DEFAULT 0.00,
    icon VARCHAR(50) DEFAULT '💰',
    icon_color VARCHAR(50) DEFAULT 'bg-blue-500',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create user_statistics table
CREATE TABLE IF NOT EXISTS user_statistics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    total_saved DECIMAL(15,2) DEFAULT 0.00,
    streak_days INTEGER DEFAULT 0,
    daily_average DECIMAL(15,2) DEFAULT 0.00,
    achievements_count INTEGER DEFAULT 0,
    last_deposit_date DATE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create achievements table
CREATE TABLE IF NOT EXISTS achievements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    icon VARCHAR(50) NOT NULL,
    icon_color VARCHAR(50) NOT NULL,
    earned_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_savings_targets_user_id ON savings_targets(user_id);
CREATE INDEX IF NOT EXISTS idx_savings_targets_is_completed ON savings_targets(is_completed);
CREATE INDEX IF NOT EXISTS idx_activities_user_id ON activities(user_id);
CREATE INDEX IF NOT EXISTS idx_activities_created_at ON activities(created_at);
CREATE INDEX IF NOT EXISTS idx_user_statistics_user_id ON user_statistics(user_id);
CREATE INDEX IF NOT EXISTS idx_achievements_user_id ON achievements(user_id);

-- Insert sample data for development
INSERT INTO savings_targets (user_id, name, target_amount, current_amount, icon, icon_color, target_date)
SELECT 
    u.id,
    unnest(ARRAY['DP Rumah', 'Mobil Keluarga', 'Liburan Raja Ampat', 'Dana Darurat']),
    unnest(ARRAY[50000000.00, 31000000.00, 15000000.00, 10000000.00]),
    unnest(ARRAY[8000000.00, 5000000.00, 200000.00, 3000000.00]),
    unnest(ARRAY['🏠', '🚗', '✈️', '💰']),
    unnest(ARRAY['bg-green-500', 'bg-red-500', 'bg-blue-500', 'bg-yellow-500']),
    (CURRENT_DATE + INTERVAL '1 year')
FROM users u 
WHERE NOT u.is_admin 
LIMIT 1;

-- Insert sample activities
INSERT INTO activities (user_id, activity_type, title, description, amount, icon, icon_color)
SELECT 
    u.id,
    unnest(ARRAY['deposit', 'target_created', 'deposit', 'achievement']),
    unnest(ARRAY['Menabung untuk DP Rumah', 'Target baru dibuat', 'Nabung otomatis harian', 'Achievement: 1 Bulan Konsisten']),
    unnest(ARRAY['Hari ini, 17:30', 'Target "Mobil Keluarga" berhasil dibuat', '2 hari lalu, 09:00', '3 hari lalu, 10:45']),
    unnest(ARRAY[100000.00, 0.00, 50000.00, 0.00]),
    unnest(ARRAY['🏠', '🎯', '💰', '🏆']),
    unnest(ARRAY['bg-green-500', 'bg-red-500', 'bg-blue-500', 'bg-yellow-500'])
FROM users u 
WHERE NOT u.is_admin 
LIMIT 1;

-- Insert sample user statistics
INSERT INTO user_statistics (user_id, total_saved, streak_days, daily_average, achievements_count, last_deposit_date)
SELECT 
    u.id,
    8750000.00,
    45,
    58000.00,
    3,
    CURRENT_DATE
FROM users u 
WHERE NOT u.is_admin;

-- Insert sample achievements
INSERT INTO achievements (user_id, title, description, icon, icon_color)
SELECT 
    u.id,
    unnest(ARRAY['Streak 10 Hari!', 'Target Master', 'RP 10M+']),
    unnest(ARRAY['Konsisten menabung 10 hari berturut-turut', 'Berhasil mencapai 3 target tabungan', 'Total tabungan yang terkumpul mencapai 10M+']),
    unnest(ARRAY['🏆', '🎯', '💰']),
    unnest(ARRAY['bg-yellow-500', 'bg-red-500', 'bg-green-500'])
FROM users u 
WHERE NOT u.is_admin 
LIMIT 1;
