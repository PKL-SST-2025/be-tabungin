-- Add reminders table for target date notifications

CREATE TABLE IF NOT EXISTS reminders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    savings_target_id UUID NOT NULL REFERENCES savings_targets(id) ON DELETE CASCADE,
    reminder_date DATE NOT NULL,
    reminder_type VARCHAR(50) NOT NULL, -- 'target_deadline', 'milestone', 'weekly_reminder'
    title VARCHAR(255) NOT NULL,
    description TEXT,
    is_completed BOOLEAN DEFAULT FALSE,
    is_notified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_reminders_user_id ON reminders(user_id);
CREATE INDEX IF NOT EXISTS idx_reminders_savings_target_id ON reminders(savings_target_id);
CREATE INDEX IF NOT EXISTS idx_reminders_reminder_date ON reminders(reminder_date);
CREATE INDEX IF NOT EXISTS idx_reminders_is_completed ON reminders(is_completed);

-- Function to automatically create reminders when savings target is created
CREATE OR REPLACE FUNCTION create_target_reminders()
RETURNS TRIGGER AS $$
BEGIN
    -- Create reminder 1 month before target date
    IF NEW.target_date IS NOT NULL AND NEW.target_date > CURRENT_DATE + INTERVAL '30 days' THEN
        INSERT INTO reminders (user_id, savings_target_id, reminder_date, reminder_type, title, description)
        VALUES (
            NEW.user_id,
            NEW.id,
            NEW.target_date - INTERVAL '30 days',
            'milestone',
            'Target ' || NEW.name || ' 1 bulan lagi!',
            'Target tabungan ' || NEW.name || ' akan berakhir dalam 1 bulan. Pastikan kamu sudah mencapai target!'
        );
    END IF;

    -- Create reminder 1 week before target date
    IF NEW.target_date IS NOT NULL AND NEW.target_date > CURRENT_DATE + INTERVAL '7 days' THEN
        INSERT INTO reminders (user_id, savings_target_id, reminder_date, reminder_type, title, description)
        VALUES (
            NEW.user_id,
            NEW.id,
            NEW.target_date - INTERVAL '7 days',
            'milestone',
            'Target ' || NEW.name || ' 1 minggu lagi!',
            'Target tabungan ' || NEW.name || ' akan berakhir dalam 1 minggu. Jangan lupa untuk menabung!'
        );
    END IF;

    -- Create reminder on target date
    IF NEW.target_date IS NOT NULL THEN
        INSERT INTO reminders (user_id, savings_target_id, reminder_date, reminder_type, title, description)
        VALUES (
            NEW.user_id,
            NEW.id,
            NEW.target_date,
            'target_deadline',
            'Target ' || NEW.name || ' berakhir hari ini!',
            'Hari ini adalah batas waktu untuk target tabungan ' || NEW.name || '. Bagaimana progresmu?'
        );
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger
CREATE TRIGGER trigger_create_target_reminders
    AFTER INSERT ON savings_targets
    FOR EACH ROW
    EXECUTE FUNCTION create_target_reminders();

-- Insert sample reminders for existing targets
INSERT INTO reminders (user_id, savings_target_id, reminder_date, reminder_type, title, description)
SELECT 
    st.user_id,
    st.id,
    unnest(ARRAY[
        st.target_date - INTERVAL '30 days',
        st.target_date - INTERVAL '7 days',
        st.target_date
    ]),
    unnest(ARRAY['milestone', 'milestone', 'target_deadline']),
    unnest(ARRAY[
        'Target ' || st.name || ' 1 bulan lagi!',
        'Target ' || st.name || ' 1 minggu lagi!',
        'Target ' || st.name || ' berakhir hari ini!'
    ]),
    unnest(ARRAY[
        'Target tabungan ' || st.name || ' akan berakhir dalam 1 bulan. Pastikan kamu sudah mencapai target!',
        'Target tabungan ' || st.name || ' akan berakhir dalam 1 minggu. Jangan lupa untuk menabung!',
        'Hari ini adalah batas waktu untuk target tabungan ' || st.name || '. Bagaimana progresmu?'
    ])
FROM savings_targets st
WHERE st.target_date IS NOT NULL
  AND st.target_date >= CURRENT_DATE
  AND NOT EXISTS (
    SELECT 1 FROM reminders r 
    WHERE r.savings_target_id = st.id
  );
