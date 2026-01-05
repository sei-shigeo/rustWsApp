-- ============================================================
-- 車両管理テーブル群 ロールバック
-- ============================================================

-- インデックスを削除
DROP INDEX IF EXISTS idx_vehicle_repair_history_date;
DROP INDEX IF EXISTS idx_vehicle_repair_history_vehicle_id;
DROP INDEX IF EXISTS idx_vehicle_inspection_history_next_date;
DROP INDEX IF EXISTS idx_vehicle_inspection_history_date;
DROP INDEX IF EXISTS idx_vehicle_inspection_history_vehicle_id;
DROP INDEX IF EXISTS idx_vehicles_voluntary_insurance_expiration;
DROP INDEX IF EXISTS idx_vehicles_insurance_expiration;
DROP INDEX IF EXISTS idx_vehicles_inspection_expiration;
DROP INDEX IF EXISTS idx_vehicles_active;
DROP INDEX IF EXISTS idx_vehicles_manufacturer_id;
DROP INDEX IF EXISTS idx_vehicles_ownership_type_id;
DROP INDEX IF EXISTS idx_vehicles_vehicle_type_id;
DROP INDEX IF EXISTS idx_vehicles_office_id;
DROP INDEX IF EXISTS idx_vehicles_company_id;

-- テーブルを逆順で削除（外部キー制約を考慮）
DROP TABLE IF EXISTS vehicle_repair_history;
DROP TABLE IF EXISTS vehicle_inspection_history;
DROP TABLE IF EXISTS vehicles;
DROP TABLE IF EXISTS vehicle_ownership_types;
DROP TABLE IF EXISTS vehicle_types;
DROP TABLE IF EXISTS vehicle_manufacturers;
