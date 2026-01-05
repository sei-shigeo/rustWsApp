-- ============================================================
-- 免許・資格・健康診断関連テーブル群 ロールバック
-- ============================================================

-- インデックスを削除
DROP INDEX IF EXISTS idx_employee_documents_active;
DROP INDEX IF EXISTS idx_employee_documents_type;
DROP INDEX IF EXISTS idx_employee_documents_employee_id;
DROP INDEX IF EXISTS idx_guidance_education_history_type_id;
DROP INDEX IF EXISTS idx_guidance_education_history_employee_id;
DROP INDEX IF EXISTS idx_aptitude_checkup_expiration_date;
DROP INDEX IF EXISTS idx_aptitude_checkup_history_type_id;
DROP INDEX IF EXISTS idx_aptitude_checkup_history_employee_id;
DROP INDEX IF EXISTS idx_health_checkup_expiration_date;
DROP INDEX IF EXISTS idx_health_checkup_history_type_id;
DROP INDEX IF EXISTS idx_health_checkup_history_employee_id;
DROP INDEX IF EXISTS idx_insurance_history_active;
DROP INDEX IF EXISTS idx_insurance_history_insurance_type_id;
DROP INDEX IF EXISTS idx_insurance_history_employee_id;
DROP INDEX IF EXISTS idx_qualifications_active;
DROP INDEX IF EXISTS idx_qualifications_expiration_date;
DROP INDEX IF EXISTS idx_qualifications_qualification_type_id;
DROP INDEX IF EXISTS idx_qualifications_employee_id;
DROP INDEX IF EXISTS idx_licenses_active;
DROP INDEX IF EXISTS idx_licenses_expiration_date;
DROP INDEX IF EXISTS idx_licenses_license_type_id;
DROP INDEX IF EXISTS idx_licenses_employee_id;

-- テーブルを逆順で削除（外部キー制約を考慮）
DROP TABLE IF EXISTS employee_documents;
DROP TABLE IF EXISTS document_types;
DROP TABLE IF EXISTS guidance_education_history;
DROP TABLE IF EXISTS aptitude_checkup_history;
DROP TABLE IF EXISTS health_checkup_history;
DROP TABLE IF EXISTS insurance_history;
DROP TABLE IF EXISTS qualifications;
DROP TABLE IF EXISTS licenses;
