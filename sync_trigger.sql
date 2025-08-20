-- Function untuk auto-sync current_amount di savings_targets dan user_statistics
CREATE OR REPLACE FUNCTION sync_target_current_amount()
RETURNS TRIGGER AS $$
DECLARE
    affected_user_id UUID;
    total_user_deposits DECIMAL;
BEGIN
    -- Get user_id from either NEW or OLD record
    affected_user_id = COALESCE(NEW.user_id, OLD.user_id);
    
    -- Calculate total deposits for this user across ALL targets
    SELECT COALESCE(SUM(amount), 0) INTO total_user_deposits
    FROM activities 
    WHERE user_id = affected_user_id
    AND activity_type = 'deposit';
    
    -- Update savings_targets current_amount for the specific target
    IF TG_OP != 'DELETE' AND NEW.savings_target_id IS NOT NULL THEN
        UPDATE savings_targets 
        SET current_amount = (
            SELECT COALESCE(SUM(amount), 0)
            FROM activities 
            WHERE savings_target_id = NEW.savings_target_id
            AND activity_type = 'deposit'
        ),
        updated_at = NOW()
        WHERE id = NEW.savings_target_id;
    END IF;
    
    -- If this is a DELETE operation, update the affected target
    IF TG_OP = 'DELETE' AND OLD.savings_target_id IS NOT NULL THEN
        UPDATE savings_targets 
        SET current_amount = (
            SELECT COALESCE(SUM(amount), 0)
            FROM activities 
            WHERE savings_target_id = OLD.savings_target_id
            AND activity_type = 'deposit'
        ),
        updated_at = NOW()
        WHERE id = OLD.savings_target_id;
    END IF;
    
    -- Update user_statistics total_saved with TOTAL across all targets
    UPDATE user_statistics 
    SET total_saved = total_user_deposits,
        updated_at = NOW()
    WHERE user_id = affected_user_id;
    
    -- Create user_statistics if doesn't exist
    IF NOT FOUND THEN
        INSERT INTO user_statistics (user_id, total_saved, streak_days, daily_average, achievements_count, last_deposit_date)
        VALUES (affected_user_id, total_user_deposits, 0, 0, 0, NULL);
    END IF;
    
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

-- Drop existing trigger if exists
DROP TRIGGER IF EXISTS trigger_sync_target_amount ON activities;

-- Create trigger for activities (simplified without WHEN clause)
CREATE TRIGGER trigger_sync_target_amount
    AFTER INSERT OR UPDATE OR DELETE ON activities
    FOR EACH ROW
    EXECUTE FUNCTION sync_target_current_amount();
