-- Update target current amount to match actual deposits
-- Fix the mismatch between current_amount in targets and total deposits

UPDATE savings_targets 
SET current_amount = (
    SELECT COALESCE(SUM(amount), 0)
    FROM activities 
    WHERE activities.savings_target_id = savings_targets.id 
    AND activity_type = 'deposit'
),
updated_at = NOW()
WHERE user_id = (SELECT id FROM users WHERE email = 'umar@app.com');

-- Verify the update
SELECT 
    st.name,
    st.current_amount as "target_current_amount",
    COALESCE(SUM(a.amount), 0) as "actual_deposits",
    st.target_amount
FROM savings_targets st
LEFT JOIN activities a ON a.savings_target_id = st.id AND a.activity_type = 'deposit'
WHERE st.user_id = (SELECT id FROM users WHERE email = 'umar@app.com')
GROUP BY st.id, st.name, st.current_amount, st.target_amount;
