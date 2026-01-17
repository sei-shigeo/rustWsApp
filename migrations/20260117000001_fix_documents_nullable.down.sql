-- ============================================================
-- employee_documentsテーブルのNOT NULL制約を復元
-- ============================================================

-- document_type_idのNOT NULL制約を復元
ALTER TABLE employee_documents
ALTER COLUMN document_type_id SET NOT NULL;

-- file_pathのNOT NULL制約を復元
ALTER TABLE employee_documents
ALTER COLUMN file_path SET NOT NULL;
