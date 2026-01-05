-- ============================================================
-- 会社・営業所・部署マスター ロールバック
-- ============================================================

-- インデックスを削除
DROP INDEX IF EXISTS idx_positions_company_id;
DROP INDEX IF EXISTS idx_departments_company_id;
DROP INDEX IF EXISTS idx_offices_company_id;
DROP INDEX IF EXISTS idx_company_bank_accounts_company_id;

-- テーブルを逆順で削除（外部キー制約を考慮）
DROP TABLE IF EXISTS positions;
DROP TABLE IF EXISTS departments;
DROP TABLE IF EXISTS offices;
DROP TABLE IF EXISTS nationalities;
DROP TABLE IF EXISTS company_bank_accounts;
DROP TABLE IF EXISTS companies;
