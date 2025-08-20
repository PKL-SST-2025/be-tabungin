-- Simple seeder untuk streak data user umar@app.com
-- Pastikan user ada terlebih dahulu

-- Insert user umar@app.com jika belum ada
INSERT INTO users (id, full_name, email, password_hash, created_at, updated_at)
VALUES (
    'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid,
    'Umar',
    'umar@app.com',
    '$2b$12$LQv3c1yX8LjjW1q0fGzDMe1qo.8y9xLV.K8zBxQzGKQJGZ5OZzJ8m', -- password: "password123"
    NOW() - INTERVAL '30 days',
    NOW()
)
ON CONFLICT (email) DO UPDATE SET
    full_name = EXCLUDED.full_name,
    updated_at = NOW();

-- Buat savings target jika belum ada
INSERT INTO savings_targets (id, user_id, name, target_amount, current_amount, icon, icon_color, target_date, is_completed, created_at, updated_at)
SELECT 
    '660e8400-e29b-41d4-a716-446655440001'::uuid,
    'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid,
    'Target Tabungan Harian',
    1000000,
    1200000,
    '💰',
    'bg-green-500',
    '2025-12-31',
    false,
    NOW() - INTERVAL '25 days',
    NOW()
WHERE NOT EXISTS (
    SELECT 1 FROM savings_targets 
    WHERE id = '660e8400-e29b-41d4-a716-446655440001'::uuid
);

-- Hapus activities lama jika ada
DELETE FROM activities WHERE user_id = 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid AND activity_type = 'deposit';

-- Insert 16 hari streak activities
INSERT INTO activities (id, user_id, savings_target_id, activity_type, title, description, amount, icon, icon_color, created_at) VALUES
('770e8400-e29b-41d4-a716-446655440001'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid, 'deposit', 'Nabung Hari Ke-1', 'Deposit harian untuk memulai streak tabungan', 75000, '💰', 'bg-green-500', '2025-07-21 08:30:00'::timestamp),
('770e8400-e29b-41d4-a716-446655440002'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid, 'deposit', 'Nabung Hari Ke-2', 'Lanjutkan streak tabungan hari kedua', 82000, '💰', 'bg-green-500', '2025-07-22 09:15:00'::timestamp),
('770e8400-e29b-41d4-a716-446655440003'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid, 'deposit', 'Nabung Hari Ke-3', 'Streak hari ketiga - tetap konsisten!', 68000, '💰', 'bg-green-500', '2025-07-23 07:45:00'::timestamp),
('770e8400-e29b-41d4-a716-446655440004'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid, 'deposit', 'Nabung Hari Ke-4', 'Empat hari berturut-turut menabung', 91000, '💰', 'bg-green-500', '2025-07-24 10:20:00'::timestamp),
('770e8400-e29b-41d4-a716-446655440005'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid, 'deposit', 'Nabung Hari Ke-5', 'Lima hari streak - pertahankan momentum!', 76000, '💰', 'bg-green-500', '2025-07-25 08:10:00'::timestamp),
('770e8400-e29b-41d4-a716-446655440006'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid, 'deposit', 'Nabung Hari Ke-6', 'Enam hari berturut-turut - hebat!', 85000, '💰', 'bg-green-500', '2025-07-26 09:30:00'::timestamp),
('770e8400-e29b-41d4-a716-446655440007'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid, 'deposit', 'Nabung Hari Ke-7', 'Satu minggu penuh streak tabungan!', 73000, '💰', 'bg-green-500', '2025-07-27 07:55:00'::timestamp),
('770e8400-e29b-41d4-a716-446655440008'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid, 'deposit', 'Nabung Hari Ke-8', 'Delapan hari berturut-turut - luar biasa!', 89000, '💰', 'bg-green-500', '2025-07-28 10:45:00'::timestamp),
('770e8400-e29b-41d4-a716-446655440009'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid, 'deposit', 'Nabung Hari Ke-9', 'Sembilan hari streak - terus lanjutkan!', 78000, '💰', 'bg-green-500', '2025-07-29 08:25:00'::timestamp),
('770e8400-e29b-41d4-a716-446655440010'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid, 'deposit', 'Nabung Hari Ke-10', 'Sepuluh hari streak - double digit!', 94000, '💰', 'bg-green-500', '2025-07-30 09:10:00'::timestamp),
('770e8400-e29b-41d4-a716-446655440011'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid, 'deposit', 'Nabung Hari Ke-11', 'Sebelas hari berturut-turut menabung', 71000, '💰', 'bg-green-500', '2025-07-31 07:40:00'::timestamp),
('770e8400-e29b-41d4-a716-446655440012'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid, 'deposit', 'Nabung Hari Ke-12', 'Masuk bulan Agustus dengan streak!', 87000, '💰', 'bg-green-500', '2025-08-01 08:50:00'::timestamp),
('770e8400-e29b-41d4-a716-446655440013'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid, 'deposit', 'Nabung Hari Ke-13', 'Tiga belas hari streak tabungan', 79000, '💰', 'bg-green-500', '2025-08-02 09:35:00'::timestamp),
('770e8400-e29b-41d4-a716-446655440014'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid, 'deposit', 'Nabung Hari Ke-14', 'Empat belas hari berturut-turut!', 92000, '💰', 'bg-green-500', '2025-08-03 10:15:00'::timestamp),
('770e8400-e29b-41d4-a716-446655440015'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid, 'deposit', 'Nabung Hari Ke-15', 'Lima belas hari streak - hampir tiga minggu!', 74000, '💰', 'bg-green-500', '2025-08-04 08:05:00'::timestamp),
('770e8400-e29b-41d4-a716-446655440016'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid, 'deposit', 'Nabung Hari Ke-16', 'Enam belas hari streak - luar biasa!', 86000, '💰', 'bg-green-500', '2025-08-05 09:20:00'::timestamp);

-- Hapus achievements lama jika ada
DELETE FROM achievements WHERE user_id = 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid;

-- Insert achievements
INSERT INTO achievements (id, user_id, title, description, icon, icon_color, earned_at) VALUES
('880e8400-e29b-41d4-a716-446655440001'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, 'Streak Master', 'Menabung 7 hari berturut-turut', '🔥', 'bg-orange-500', '2025-07-27 18:00:00'::timestamp),
('880e8400-e29b-41d4-a716-446655440002'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, 'Konsisten Banget', 'Menabung 14 hari berturut-turut', '⭐', 'bg-yellow-500', '2025-08-03 18:00:00'::timestamp),
('880e8400-e29b-41d4-a716-446655440003'::uuid, 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid, 'Jutawan Muda', 'Berhasil menabung lebih dari 1 juta rupiah', '💎', 'bg-purple-500', '2025-08-05 20:00:00'::timestamp);

-- Update user statistics
DELETE FROM user_statistics WHERE user_id = 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid;

INSERT INTO user_statistics (id, user_id, total_saved, streak_days, daily_average, achievements_count, last_deposit_date, created_at, updated_at)
VALUES (
    '990e8400-e29b-41d4-a716-446655440001'::uuid,
    'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid,
    1302000, -- Total dari semua deposit
    16, -- 16 hari streak
    81375, -- Average per hari (1302000/16)
    3, -- 3 achievements
    '2025-08-05'::date,
    NOW() - INTERVAL '25 days',
    NOW()
);

-- Hapus reminder lama jika ada
DELETE FROM reminders WHERE user_id = 'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid;

-- Buat reminder untuk hari ini agar streak tetap berlanjut
INSERT INTO reminders (id, user_id, savings_target_id, reminder_date, reminder_type, title, description, is_completed, is_notified, created_at, updated_at)
VALUES (
    'aa0e8400-e29b-41d4-a716-446655440001'::uuid,
    'e4c13900-95cb-4fcb-a55d-8432a768c35c'::uuid,
    '660e8400-e29b-41d4-a716-446655440001'::uuid,
    CURRENT_DATE,
    'daily_saving',
    'Waktunya Menabung!',
    'Jangan sampai streak 16 hari kamu terputus! Ayo nabung hari ini.',
    false,
    false,
    NOW(),
    NOW()
);
