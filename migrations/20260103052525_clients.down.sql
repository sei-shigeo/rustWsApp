-- ============================================================
-- 取引先管理テーブル群 ロールバック
-- ============================================================

-- インデックスを削除
DROP INDEX IF EXISTS idx_client_contacts_client_office_id;
DROP INDEX IF EXISTS idx_client_contacts_client_id;
DROP INDEX IF EXISTS idx_client_offices_client_id;
DROP INDEX IF EXISTS idx_clients_active;
DROP INDEX IF EXISTS idx_clients_client_type_id;
DROP INDEX IF EXISTS idx_clients_company_id;

-- テーブルを逆順で削除（外部キー制約を考慮）
DROP TABLE IF EXISTS client_contacts;
DROP TABLE IF EXISTS client_offices;
DROP TABLE IF EXISTS clients;
DROP TABLE IF EXISTS client_types;
