-- Add nomor_telepon, alamat, posisi_jabatan to users table
ALTER TABLE users
ADD COLUMN nomor_telepon VARCHAR(50),
ADD COLUMN alamat TEXT,
ADD COLUMN posisi_jabatan VARCHAR(100);
