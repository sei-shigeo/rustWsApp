-- ============================================================
-- 従業員基本情報テーブル ロールバック
-- ============================================================

-- インデックスを削除
DROP INDEX IF EXISTS idx_department_position_history_current;
DROP INDEX IF EXISTS idx_department_position_history_employee_id;
DROP INDEX IF EXISTS idx_employee_bank_accounts_primary;
DROP INDEX IF EXISTS idx_employee_bank_accounts_active;
DROP INDEX IF EXISTS idx_employee_bank_accounts_employee_id;
DROP INDEX IF EXISTS idx_education_history_employee_id;
DROP INDEX IF EXISTS idx_employment_history_employee_id;
DROP INDEX IF EXISTS idx_addresses_current;
DROP INDEX IF EXISTS idx_addresses_employee_id;
DROP INDEX IF EXISTS idx_emergency_contacts_active;
DROP INDEX IF EXISTS idx_emergency_contacts_employee_id;
DROP INDEX IF EXISTS idx_residence_cards_active;
DROP INDEX IF EXISTS idx_residence_cards_expiration_date;
DROP INDEX IF EXISTS idx_residence_cards_employee_id;
DROP INDEX IF EXISTS idx_employees_name;
DROP INDEX IF EXISTS idx_employees_active;
DROP INDEX IF EXISTS idx_employees_employee_code;
DROP INDEX IF EXISTS idx_employees_position_id;
DROP INDEX IF EXISTS idx_employees_department_id;
DROP INDEX IF EXISTS idx_employees_office_id;
DROP INDEX IF EXISTS idx_employees_nationality_id;
DROP INDEX IF EXISTS idx_employees_company_id;

-- テーブルを逆順で削除（外部キー制約を考慮）
DROP TABLE IF EXISTS department_position_history;
DROP TABLE IF EXISTS employee_bank_accounts;
DROP TABLE IF EXISTS education_history;
DROP TABLE IF EXISTS employment_history;
DROP TABLE IF EXISTS addresses;
DROP TABLE IF EXISTS emergency_contacts;
DROP TABLE IF EXISTS residence_cards;
DROP TABLE IF EXISTS employees;
