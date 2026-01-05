-- マイグレーション履歴のみをリセットするスクリプト
-- データベースを削除せずに、マイグレーション履歴だけをクリアします
--
-- 使用方法:
--   方法1: psql コマンドで実行
--     psql -U postgres -h localhost -d rustWsApp -f scripts/reset_migrations_only.sql
--
--   方法2: VS Code / DBeaver / pgAdmin 等で実行
--     1. rustWsAppデータベースに接続
--     2. このファイルの内容を全選択してコピー
--     3. SQLエディタに貼り付けて実行
--
-- 注意: このスクリプトは全てのテーブルを削除しますが、データベース自体は削除しません

-- ============================================================
-- Step 1: マイグレーション履歴テーブルの確認
-- ============================================================

DO $$
BEGIN
    RAISE NOTICE '現在のマイグレーション履歴:';
END $$;

SELECT version, description, installed_on
FROM _sqlx_migrations
ORDER BY version;

-- ============================================================
-- Step 2: 全てのテーブルを削除（カスケード）
-- ============================================================

DO $$
BEGIN
    RAISE NOTICE '全てのテーブルを削除中...';
END $$;

-- 外部キー制約があっても削除できるようにカスケード削除

-- 車両管理テーブル
DROP TABLE IF EXISTS vehicle_repair_history CASCADE;
DROP TABLE IF EXISTS vehicle_inspection_history CASCADE;
DROP TABLE IF EXISTS vehicles CASCADE;
DROP TABLE IF EXISTS vehicle_ownership_types CASCADE;
DROP TABLE IF EXISTS vehicle_types CASCADE;
DROP TABLE IF EXISTS vehicle_manufacturers CASCADE;

-- 免許・資格・健康診断テーブル
DROP TABLE IF EXISTS employee_documents CASCADE;
DROP TABLE IF EXISTS document_types CASCADE;
DROP TABLE IF EXISTS guidance_education_history CASCADE;
DROP TABLE IF EXISTS aptitude_checkup_history CASCADE;
DROP TABLE IF EXISTS health_checkup_history CASCADE;
DROP TABLE IF EXISTS insurance_history CASCADE;
DROP TABLE IF EXISTS qualifications CASCADE;
DROP TABLE IF EXISTS licenses CASCADE;

-- 従業員基本情報テーブル
DROP TABLE IF EXISTS department_position_history CASCADE;
DROP TABLE IF EXISTS employee_bank_accounts CASCADE;
DROP TABLE IF EXISTS education_history CASCADE;
DROP TABLE IF EXISTS employment_history CASCADE;
DROP TABLE IF EXISTS addresses CASCADE;
DROP TABLE IF EXISTS emergency_contacts CASCADE;
DROP TABLE IF EXISTS residence_cards CASCADE;
DROP TABLE IF EXISTS employees CASCADE;

-- 取引先管理テーブル
DROP TABLE IF EXISTS client_contacts CASCADE;
DROP TABLE IF EXISTS client_offices CASCADE;
DROP TABLE IF EXISTS clients CASCADE;
DROP TABLE IF EXISTS client_types CASCADE;

-- マスターテーブル（従業員関連）
DROP TABLE IF EXISTS health_checkup_types CASCADE;
DROP TABLE IF EXISTS aptitude_checkup_types CASCADE;
DROP TABLE IF EXISTS guidance_education_types CASCADE;
DROP TABLE IF EXISTS insurance_types CASCADE;
DROP TABLE IF EXISTS qualification_types CASCADE;
DROP TABLE IF EXISTS license_types CASCADE;
DROP TABLE IF EXISTS residence_card_types CASCADE;

-- 会社・組織マスターテーブル
DROP TABLE IF EXISTS positions CASCADE;
DROP TABLE IF EXISTS departments CASCADE;
DROP TABLE IF EXISTS offices CASCADE;
DROP TABLE IF EXISTS nationalities CASCADE;
DROP TABLE IF EXISTS company_bank_accounts CASCADE;
DROP TABLE IF EXISTS companies CASCADE;

-- マイグレーション履歴テーブル
DROP TABLE IF EXISTS _sqlx_migrations CASCADE;

-- ============================================================
-- Step 3: 削除確認
-- ============================================================

DO $$
BEGIN
    RAISE NOTICE 'テーブル削除完了';
    RAISE NOTICE '残っているテーブルを確認:';
END $$;

SELECT
    schemaname,
    tablename
FROM pg_tables
WHERE schemaname = 'public'
ORDER BY tablename;

-- ============================================================
-- Step 4: 完了メッセージ
-- ============================================================

DO $$
BEGIN
    RAISE NOTICE '================================================';
    RAISE NOTICE 'マイグレーションリセット完了！';
    RAISE NOTICE '================================================';
    RAISE NOTICE '';
    RAISE NOTICE '次のステップ:';
    RAISE NOTICE '  ターミナルで以下のコマンドを実行してください:';
    RAISE NOTICE '  cd rustWsApp';
    RAISE NOTICE '  sqlx migrate run';
    RAISE NOTICE '';
END $$;
