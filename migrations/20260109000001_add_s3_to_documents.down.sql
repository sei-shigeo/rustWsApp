-- ============================================================
-- S3対応のためのemployee_documentsテーブル更新のロールバック
-- ============================================================

-- インデックスを削除
DROP INDEX IF EXISTS idx_employee_documents_s3_key;
DROP INDEX IF EXISTS idx_employee_documents_related_id;
DROP INDEX IF EXISTS idx_employee_documents_category;

-- 追加したカラムを削除
ALTER TABLE employee_documents
DROP COLUMN IF EXISTS uploaded_at,
DROP COLUMN IF EXISTS related_id,
DROP COLUMN IF EXISTS s3_url,
DROP COLUMN IF EXISTS s3_key,
DROP COLUMN IF EXISTS category;
