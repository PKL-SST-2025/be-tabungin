-- Fix data consistency issues
-- 1. Clear existing inconsistent data
-- 2. Create consistent 16-day streak data
-- 3. Update reminders to match activities
-- 4. Fix achievements and statistics

-- Clear existing data first
DELETE FROM activities WHERE user_id = (SELECT id FROM users WHERE email = 'umar@app.com');
DELETE FROM reminders WHERE user_id = (SELECT id FROM users WHERE email = 'umar@app.com');
DELETE FROM achievements WHERE user_id = (SELECT id FROM users WHERE email = 'umar@app.com');
DELETE FROM user_statistics WHERE user_id = (SELECT id FROM users WHERE email = 'umar@app.com');
DELETE FROM savings_targets WHERE user_id = (SELECT id FROM users WHERE email = 'umar@app.com');

-- Get user ID
DO $$
DECLARE
    user_uuid UUID;
    target_uuid UUID;
    current_date_val DATE;
    i INTEGER;
BEGIN
    -- Get user ID
    SELECT id INTO user_uuid FROM users WHERE email = 'umar@app.com';
    
    IF user_uuid IS NULL THEN
        RAISE EXCEPTION 'User umar@app.com not found';
    END IF;
    
    -- Create savings target first
    INSERT INTO savings_targets (
        id, user_id, name, target_amount, current_amount, 
        icon, icon_color, target_date, created_at, updated_at
    ) VALUES (
        gen_random_uuid(), user_uuid, 'Dana Darurat', 5000000, 3200000,
        '🛡️', 'bg-blue-500', '2025-12-31'::date,
        NOW(), NOW()
    ) RETURNING id INTO target_uuid;
    
    -- Create 16 consecutive days of activities (July 24 - August 8, 2025)
    FOR i IN 0..15 LOOP
        current_date_val := '2025-07-24'::date + (i || ' days')::interval;
        
        -- Insert deposit activity
        INSERT INTO activities (
            id, user_id, savings_target_id, activity_type, title, description,
            amount, icon, icon_color, created_at
        ) VALUES (
            gen_random_uuid(), user_uuid, target_uuid, 'deposit',
            'Nabung Harian ke-' || (i + 1),
            'Deposit rutin untuk dana darurat',
            200000, '💰', 'bg-green-500',
            current_date_val + '10:00:00'::time
        );
        
        -- Create reminder for each deposit (marked as completed)
        INSERT INTO reminders (
            id, user_id, savings_target_id, reminder_date, reminder_type,
            title, description, is_completed, created_at, updated_at
        ) VALUES (
            gen_random_uuid(), user_uuid, target_uuid,
            current_date_val, 'weekly_reminder',
            'Nabung Dana Darurat',
            'Reminder untuk menabung harian - Sudah Selesai',
            true, -- Mark as completed since we have activities
            current_date_val + '09:00:00'::time,
            current_date_val + '09:00:00'::time
        );
    END LOOP;
    
    -- Add future reminders (not completed)
    FOR i IN 1..7 LOOP
        current_date_val := '2025-08-08'::date + (i || ' days')::interval;
        
        INSERT INTO reminders (
            id, user_id, savings_target_id, reminder_date, reminder_type,
            title, description, is_completed, created_at, updated_at
        ) VALUES (
            gen_random_uuid(), user_uuid, target_uuid,
            current_date_val, 'weekly_reminder',
            'Nabung Dana Darurat',
            'Reminder untuk menabung harian',
            false, -- Not completed yet
            NOW(), NOW()
        );
    END LOOP;
    
    -- Create achievements
    INSERT INTO achievements (
        id, user_id, title, description,
        icon, icon_color, earned_at
    ) VALUES 
    (
        gen_random_uuid(), user_uuid,
        'Langkah Pertama', 'Berhasil melakukan deposit pertama',
        '🌟', 'bg-yellow-500', '2025-07-24 10:00:00'
    ),
    (
        gen_random_uuid(), user_uuid,
        'Konsisten Seminggu', 'Menabung 7 hari berturut-turut',
        '🔥', 'bg-orange-500', '2025-07-30 10:00:00'
    ),
    (
        gen_random_uuid(), user_uuid,
        'Dua Minggu Berapi', 'Menabung 14 hari berturut-turut',
        '💎', 'bg-purple-500', '2025-08-06 10:00:00'
    );
    
    -- Insert user statistics
    INSERT INTO user_statistics (
        id, user_id, total_saved, streak_days, daily_average, achievements_count, 
        last_deposit_date, created_at, updated_at
    ) VALUES (
        gen_random_uuid(), user_uuid, 3200000, 16, 200000, 3,
        '2025-08-08'::date, NOW(), NOW()
    );
    
END $$;

-- Verify data
SELECT 
    'Activities' as type,
    COUNT(*) as count,
    MIN(created_at)::date as start_date,
    MAX(created_at)::date as end_date
FROM activities 
WHERE user_id = (SELECT id FROM users WHERE email = 'umar@app.com')

UNION ALL

SELECT 
    'Reminders' as type,
    COUNT(*) as count,
    MIN(reminder_date) as start_date,
    MAX(reminder_date) as end_date
FROM reminders 
WHERE user_id = (SELECT id FROM users WHERE email = 'umar@app.com')

UNION ALL

SELECT 
    'Achievements' as type,
    COUNT(*) as count,
    MIN(earned_at)::date as start_date,
    MAX(earned_at)::date as end_date
FROM achievements 
WHERE user_id = (SELECT id FROM users WHERE email = 'umar@app.com');
