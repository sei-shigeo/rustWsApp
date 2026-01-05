-- 従業員コードを全社で一意にする
-- 既存の会社別制約を削除して、グローバルな一意制約を追加

-- 既存の制約を削除
ALTER TABLE employees DROP CONSTRAINT IF EXISTS uq_employees_company_code;

-- 新しい制約: employee_codeがNULLでない場合は一意
CREATE UNIQUE INDEX uq_employees_employee_code
ON employees(employee_code)
WHERE employee_code IS NOT NULL;
