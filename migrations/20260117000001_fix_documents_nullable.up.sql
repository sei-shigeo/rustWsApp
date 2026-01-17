-- ============================================================
-- employee_documentsテーブルのNOT NULL制約を修正
-- S3ベースのアップロードではdocument_type_idとfile_pathは不要
-- ============================================================

-- document_type_idのNOT NULL制約を削除
ALTER TABLE employee_documents
ALTER COLUMN document_type_id DROP NOT NULL;

-- file_pathのNOT NULL制約を削除
ALTER TABLE employee_documents
ALTER COLUMN file_path DROP NOT NULL;
