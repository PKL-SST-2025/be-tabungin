-- Seed data untuk testing streak functionality
-- Insert activities untuk user dengan streak pattern yang realistis

-- Insert activities untuk 16 hari terakhir (membuat streak 16 hari)
-- Menggunakan user_id dari user yang ada (ambil dari tabel users)

-- Insert activities untuk streak (tabel activities tidak memiliki updated_at)
INSERT INTO activities (user_id, activity_type, title, description, amount, icon, icon_color, created_at)
SELECT 
    u.id,
    'deposit',
    'Deposit harian - streak hari ' || generate_series(1,16),
    'Deposit rutin untuk membangun streak tabungan',
    (RANDOM() * 50000 + 50000)::DECIMAL(15,2), -- Random amount between 50k-100k
    '💰',
    'bg-green-500',
    NOW() - (generate_series(16,1,-1) || ' days')::INTERVAL
FROM users u 
WHERE NOT u.is_admin 
LIMIT 1;

-- Tambahkan beberapa achievement untuk melengkapi data (tabel achievements memiliki earned_at bukan achieved_at)
INSERT INTO achievements (user_id, title, description, icon, icon_color, earned_at)
SELECT 
    u.id,
    unnest(ARRAY['Streak 7 Hari', 'Streak 14 Hari', 'Deposit Pertama']),
    unnest(ARRAY['Berhasil menabung selama 7 hari berturut-turut', 'Berhasil menabung selama 14 hari berturut-turut', 'Selamat! Anda telah melakukan deposit pertama']),
    unnest(ARRAY['🔥', '⚡', '🎉']),
    unnest(ARRAY['bg-orange-500', 'bg-yellow-500', 'bg-purple-500']),
    unnest(ARRAY[NOW() - INTERVAL '9 days', NOW() - INTERVAL '2 days', NOW() - INTERVAL '16 days'])
FROM users u 
WHERE NOT u.is_admin 
LIMIT 1;

-- Insert beberapa target untuk melengkapi
INSERT INTO savings_targets (user_id, name, target_amount, current_amount, icon, icon_color, target_date, created_at, updated_at)
SELECT 
    u.id,
    unnest(ARRAY['Liburan Keluarga', 'Smartphone Baru']),
    unnest(ARRAY[5000000.00, 8000000.00]),
    unnest(ARRAY[1200000.00, 950000.00]),
    unnest(ARRAY['🏖️', '📱']),
    unnest(ARRAY['bg-blue-500', 'bg-gray-700']),
    unnest(ARRAY['2025-12-31'::DATE, '2025-10-15'::DATE]),
    NOW() - INTERVAL '15 days',
    NOW()
FROM users u 
WHERE NOT u.is_admin 
LIMIT 1;

-- Update user statistics untuk mencerminkan data yang baru
UPDATE user_statistics 
SET 
    total_saved = (SELECT COALESCE(SUM(amount), 0) FROM activities WHERE user_id = user_statistics.user_id AND activity_type = 'deposit'),
    streak_days = 16,
    daily_average = 65000.00,
    achievements_count = (SELECT COUNT(*) FROM achievements WHERE user_id = user_statistics.user_id),
    last_deposit_date = CURRENT_DATE
WHERE user_id IN (SELECT id FROM users WHERE NOT is_admin LIMIT 1);
