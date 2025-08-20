-- Seed testimoni data
-- This migration adds dummy testimoni data for testing

-- First, let's add some dummy users if they don't exist
INSERT INTO users (id, full_name, email, password_hash, avatar, is_admin, created_at, updated_at)
VALUES 
    ('550e8400-e29b-41d4-a716-446655440001', 'Sari Wijaya', 'sari.wijaya@gmail.com', 'password123', NULL, FALSE, NOW() - INTERVAL '30 days', NOW() - INTERVAL '30 days'),
    ('550e8400-e29b-41d4-a716-446655440002', 'Budi Santoso', 'budi.santoso@yahoo.com', 'password123', NULL, FALSE, NOW() - INTERVAL '25 days', NOW() - INTERVAL '25 days'),
    ('550e8400-e29b-41d4-a716-446655440003', 'Andi Pratama', 'andi.pratama@gmail.com', 'password123', NULL, FALSE, NOW() - INTERVAL '20 days', NOW() - INTERVAL '20 days'),
    ('550e8400-e29b-41d4-a716-446655440004', 'Maya Sari', 'maya.sari@outlook.com', 'password123', NULL, FALSE, NOW() - INTERVAL '18 days', NOW() - INTERVAL '18 days'),
    ('550e8400-e29b-41d4-a716-446655440005', 'Rizki Firmansyah', 'rizki.firmansyah@gmail.com', 'password123', NULL, FALSE, NOW() - INTERVAL '15 days', NOW() - INTERVAL '15 days'),
    ('550e8400-e29b-41d4-a716-446655440006', 'Dewi Lestari', 'dewi.lestari@yahoo.com', 'password123', NULL, FALSE, NOW() - INTERVAL '12 days', NOW() - INTERVAL '12 days'),
    ('550e8400-e29b-41d4-a716-446655440007', 'Arif Rahman', 'arif.rahman@gmail.com', 'password123', NULL, FALSE, NOW() - INTERVAL '10 days', NOW() - INTERVAL '10 days'),
    ('550e8400-e29b-41d4-a716-446655440008', 'Lisa Indrawati', 'lisa.indrawati@gmail.com', 'password123', NULL, FALSE, NOW() - INTERVAL '8 days', NOW() - INTERVAL '8 days'),
    ('550e8400-e29b-41d4-a716-446655440009', 'Joko Widodo', 'joko.widodo@yahoo.com', 'password123', NULL, FALSE, NOW() - INTERVAL '6 days', NOW() - INTERVAL '6 days'),
    ('550e8400-e29b-41d4-a716-446655440010', 'Sinta Maharani', 'sinta.maharani@outlook.com', 'password123', NULL, FALSE, NOW() - INTERVAL '4 days', NOW() - INTERVAL '4 days')
ON CONFLICT (email) DO NOTHING;

-- Now insert testimoni data
INSERT INTO testimoni (id, user_id, content, rating, is_approved, created_at, updated_at)
VALUES 
    -- Rating 5 testimoni (approved)
    (
        gen_random_uuid(),
        '550e8400-e29b-41d4-a716-446655440001',
        'Aplikasi Tabungin sangat membantu saya dalam mengatur keuangan! Fitur target tabungan membuat saya lebih disiplin menabung. Dalam 3 bulan, saya berhasil mencapai target untuk liburan keluarga. Interface-nya mudah dipahami dan notifikasinya membantu mengingatkan untuk menabung setiap hari.',
        5,
        TRUE,
        NOW() - INTERVAL '5 days',
        NOW() - INTERVAL '5 days'
    ),
    (
        gen_random_uuid(),
        '550e8400-e29b-41d4-a716-446655440002',
        'Luar biasa! Tabungin benar-benar mengubah kebiasaan finansial saya. Dulu saya sulit menabung karena tidak ada target yang jelas. Sekarang dengan fitur tracking dan analisis pengeluaran, saya bisa melihat progress tabungan secara real-time. Sangat recommended untuk yang ingin hidup lebih teratur!',
        5,
        TRUE,
        NOW() - INTERVAL '7 days',
        NOW() - INTERVAL '7 days'
    ),
    (
        gen_random_uuid(),
        '550e8400-e29b-41d4-a716-446655440003',
        'Fitur-fitur di Tabungin sangat lengkap dan sesuai kebutuhan. Yang paling saya suka adalah fitur challenge menabung dan sistem reward-nya. Membuat aktivitas menabung jadi lebih menyenangkan dan tidak terasa seperti beban. Customer service-nya juga responsif ketika ada pertanyaan.',
        5,
        TRUE,
        NOW() - INTERVAL '10 days',
        NOW() - INTERVAL '10 days'
    ),
    (
        gen_random_uuid(),
        '550e8400-e29b-41d4-a716-446655440004',
        'Sebagai seorang ibu rumah tangga, Tabungin membantu saya mengatur budget keluarga dengan lebih baik. Fitur kategori pengeluaran dan laporan bulanan membuat saya bisa melihat kemana saja uang dihabiskan. Anak-anak juga jadi ikut belajar menabung dengan fitur family planning-nya.',
        5,
        TRUE,
        NOW() - INTERVAL '12 days',
        NOW() - INTERVAL '12 days'
    ),
    
    -- Rating 4 testimoni (approved)
    (
        gen_random_uuid(),
        '550e8400-e29b-41d4-a716-446655440005',
        'Aplikasi yang sangat membantu untuk planning keuangan jangka panjang. Saya sudah pakai 6 bulan dan berhasil kumpulin dana untuk DP motor. Fitur reminder dan target otomatis sangat berguna. Mungkin bisa ditambah fitur investasi untuk planning yang lebih advance.',
        4,
        TRUE,
        NOW() - INTERVAL '14 days',
        NOW() - INTERVAL '14 days'
    ),
    (
        gen_random_uuid(),
        '550e8400-e29b-41d4-a716-446655440006',
        'Interface Tabungin clean dan user-friendly. Fitur analisis pengeluaran cukup detail dan membantu mengidentifikasi spending habits yang buruk. Sistem notifikasi juga tidak terlalu spammy. Overall bagus, tapi kalau bisa ditambah fitur sync dengan mobile banking akan lebih praktis.',
        4,
        TRUE,
        NOW() - INTERVAL '16 days',
        NOW() - INTERVAL '16 days'
    ),
    (
        gen_random_uuid(),
        '550e8400-e29b-41d4-a716-446655440007',
        'Tabungin membantu saya lebih konsisten dalam menabung. Fitur goal setting-nya memotivasi untuk terus mencapai target. Dashboard-nya informatif dan mudah dimengerti. Sempat ada bug kecil di awal, tapi tim support cepat tanggap dan langsung diperbaiki.',
        4,
        TRUE,
        NOW() - INTERVAL '18 days',
        NOW() - INTERVAL '18 days'
    ),
    
    -- Rating 3 testimoni (approved)
    (
        gen_random_uuid(),
        '550e8400-e29b-41d4-a716-446655440008',
        'Aplikasi cukup baik untuk pemula yang ingin belajar menabung. Fitur-fiturnya standar tapi cukup memadai. Loading kadang agak lambat dan perlu improvement di bagian UX. Tapi overall membantu untuk memulai kebiasaan menabung yang lebih teratur.',
        3,
        TRUE,
        NOW() - INTERVAL '20 days',
        NOW() - INTERVAL '20 days'
    ),
    
    -- Rating 5 testimoni (belum approved)
    (
        gen_random_uuid(),
        '550e8400-e29b-41d4-a716-446655440009',
        'Sungguh aplikasi yang revolusioner! Tabungin tidak hanya membantu menabung tapi juga mengedukasi tentang financial literacy. Fitur tips dan artikel-artikel finansial sangat bermanfaat. Saya jadi lebih paham tentang perencanaan keuangan yang sehat.',
        5,
        FALSE,
        NOW() - INTERVAL '2 days',
        NOW() - INTERVAL '2 days'
    ),
    (
        gen_random_uuid(),
        '550e8400-e29b-41d4-a716-446655440010',
        'Aplikasi Tabungin benar-benar game changer untuk saya! Dulu saya tipe orang yang boros dan sulit menyimpan uang. Dengan fitur automatic saving dan smart budgeting, sekarang saya punya tabungan yang lumayan. Terima kasih Tabungin!',
        5,
        FALSE,
        NOW() - INTERVAL '1 day',
        NOW() - INTERVAL '1 day'
    ),
    
    -- Rating 4 testimoni (belum approved)
    (
        gen_random_uuid(),
        '550e8400-e29b-41d4-a716-446655440001',
        'Update terbaru Tabungin semakin bagus! Fitur budgeting per kategori sangat membantu tracking pengeluaran bulanan. Performance aplikasi juga semakin smooth. Keep up the good work tim developer!',
        4,
        FALSE,
        NOW() - INTERVAL '3 days',
        NOW() - INTERVAL '3 days'
    ),
    
    -- Rating 3 testimoni (belum approved)
    (
        gen_random_uuid(),
        '550e8400-e29b-41d4-a716-446655440002',
        'Aplikasi lumayan bagus untuk tracking tabungan. Fitur basic-nya sudah cukup lengkap. Tapi masih butuh improvement di bagian security dan mungkin bisa ditambah fitur social sharing progress untuk motivasi lebih.',
        3,
        FALSE,
        NOW() - INTERVAL '6 hours',
        NOW() - INTERVAL '6 hours'
    ),
    
    -- Rating 5 testimoni (approved) - Additional positive reviews
    (
        gen_random_uuid(),
        '550e8400-e29b-41d4-a716-446655440003',
        'Setelah 8 bulan menggunakan Tabungin, saya berhasil mengumpulkan dana darurat sebesar 6 bulan gaji. Fitur auto-deduct dan smart reminder sangat membantu disiplin finansial. Aplikasi ini wajib dimiliki siapa saja yang serius ingin mengatur keuangan!',
        5,
        TRUE,
        NOW() - INTERVAL '22 days',
        NOW() - INTERVAL '22 days'
    ),
    (
        gen_random_uuid(),
        '550e8400-e29b-41d4-a716-446655440004',
        'Tabungin mengubah mindset saya tentang uang! Fitur education center-nya memberikan insight berharga tentang perencanaan finansial. Dashboard analytics-nya detail banget, bisa lihat pattern spending dari waktu ke waktu. Highly recommended!',
        5,
        TRUE,
        NOW() - INTERVAL '24 days',
        NOW() - INTERVAL '24 days'
    ),
    
    -- Rating 4 testimoni (approved) - Additional good reviews
    (
        gen_random_uuid(),
        '550e8400-e29b-41d4-a716-446655440005',
        'Fitur goal-based saving di Tabungin sangat memotivasi. Bisa set multiple goals dengan timeline berbeda-beda. Visual progress bar-nya juga menarik. Cuma agak butuh waktu untuk terbiasa dengan semua fiturnya karena cukup banyak.',
        4,
        TRUE,
        NOW() - INTERVAL '26 days',
        NOW() - INTERVAL '26 days'
    )
ON CONFLICT (id) DO NOTHING;
