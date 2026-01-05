-- マイグレーション問題の修正用SQLファイル
-- 使用方法: このファイルをPostgreSQLに直接実行してください
--
-- 方法1: コマンドラインから
--   psql -U postgres -h localhost -d rustWsApp -f scripts/fix_migration.sql
--
-- 方法2: DBeaver/pgAdmin等のGUIツールで
--   1. rustWsAppデータベースに接続
--   2. このファイルの内容をコピー＆ペーストして実行
--
-- 方法3: VS Code PostgreSQL拡張から
--   1. データベースに接続
--   2. このファイルを開いて実行

-- ============================================================
-- Step 1: 現在のマイグレーション履歴を確認
-- ============================================================

SELECT
    version,
    description,
    installed_on,
    execution_time
FROM _sqlx_migrations
ORDER BY version;

-- ============================================================
-- Step 2: 古いマイグレーション履歴を削除
-- ============================================================

-- 問題のある古いマイグレーション (20260103052522) を削除
DELETE FROM _sqlx_migrations
WHERE version = 20260103052522;

-- ============================================================
-- Step 3: 削除後の確認
-- ============================================================

SELECT
    version,
    description,
    installed_on
FROM _sqlx_migrations
ORDER BY version;

-- 結果が空（または古いバージョンが削除されている）ことを確認
--
-- 次のステップ:
-- 1. このSQLファイルを実行後、ターミナルで以下を実行:
--    cd rustWsApp
--    sqlx migrate run
--
-- 2. 成功すれば以下の6つのマイグレーションが適用されます:
--    - 20260103052523_company_master
--    - 20260103052524_master_tables
--    - 20260103052525_clients
--    - 20260103052526_employees
--    - 20260103052527_licenses_qualifications
--    - 20260103052528_vehicles
