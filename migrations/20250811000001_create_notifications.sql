-- Migration: create notifications table
CREATE TABLE notifications (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    message TEXT NOT NULL,
    type VARCHAR(16) NOT NULL DEFAULT 'info',
    read BOOLEAN NOT NULL DEFAULT FALSE,
    timestamp TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);


-- Pastikan user umar@app.com ada

-- Seed notifikasi untuk user umar@app.com
INSERT INTO notifications (user_id, message, type, read, timestamp)
VALUES
    ('e4c13900-95cb-4fcb-a55d-8432a768c35c', 'Selamat datang di Tabungin!', 'success', FALSE, NOW()),
    ('e4c13900-95cb-4fcb-a55d-8432a768c35c', 'Tabungan Anda telah bertambah.', 'info', FALSE, NOW()),
    ('e4c13900-95cb-4fcb-a55d-8432a768c35c', 'Transaksi gagal, silakan coba lagi.', 'error', FALSE, NOW());
