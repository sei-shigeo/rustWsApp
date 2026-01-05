-- ロールバック: グローバル一意制約を削除して、元の会社別制約に戻す

-- グローバル制約を削除
DROP INDEX IF EXISTS uq_employees_employee_code;

-- 元の会社別制約を復元
ALTER TABLE employees
ADD CONSTRAINT uq_employees_company_code
UNIQUE NULLS NOT DISTINCT (company_id, employee_code);
