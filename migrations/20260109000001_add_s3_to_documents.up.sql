-- ============================================================
-- S3対応のためのemployee_documentsテーブル更新
-- ============================================================

-- カテゴリカラムを追加（document_type_idの代わりに使用可能）
ALTER TABLE employee_documents
ADD COLUMN IF NOT EXISTS category VARCHAR(50);

-- S3関連のカラムを追加
ALTER TABLE employee_documents
ADD COLUMN IF NOT EXISTS s3_key VARCHAR(500),
ADD COLUMN IF NOT EXISTS s3_url VARCHAR(1000),
ADD COLUMN IF NOT EXISTS related_id INTEGER;

-- uploaded_atカラムを追加（upload_dateとは別に時刻情報も保持）
ALTER TABLE employee_documents
ADD COLUMN IF NOT EXISTS uploaded_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL;

-- 既存データのマイグレーション
UPDATE employee_documents
SET category = CASE
    WHEN document_type_id IN (SELECT id FROM document_types WHERE name LIKE '%顔写真%') THEN 'FACE_PHOTO'
    WHEN document_type_id IN (SELECT id FROM document_types WHERE name LIKE '%免許証%表%') THEN 'LICENSE_FRONT'
    WHEN document_type_id IN (SELECT id FROM document_types WHERE name LIKE '%免許証%裏%') THEN 'LICENSE_BACK'
    WHEN document_type_id IN (SELECT id FROM document_types WHERE name LIKE '%資格証%') THEN 'QUALIFICATION_CERT'
    WHEN document_type_id IN (SELECT id FROM document_types WHERE name LIKE '%車検証%') THEN 'VEHICLE_INSPECTION'
    WHEN document_type_id IN (SELECT id FROM document_types WHERE name LIKE '%健康診断%') THEN 'HEALTH_CHECKUP'
    WHEN document_type_id IN (SELECT id FROM document_types WHERE name LIKE '%適性診断%') THEN 'APTITUDE_CHECKUP'
    ELSE 'OTHER'
END
WHERE category IS NULL;

-- s3_keyとs3_urlを既存のfile_pathから生成（必要に応じて）
UPDATE employee_documents
SET
    s3_key = file_path,
    s3_url = file_path
WHERE s3_key IS NULL AND file_path IS NOT NULL;

-- uploaded_atを既存のupload_dateから設定
UPDATE employee_documents
SET uploaded_at = upload_date::timestamptz
WHERE uploaded_at IS NULL AND upload_date IS NOT NULL;

-- インデックスを追加
CREATE INDEX IF NOT EXISTS idx_employee_documents_category
ON employee_documents(employee_id, category);

CREATE INDEX IF NOT EXISTS idx_employee_documents_related_id
ON employee_documents(employee_id, related_id)
WHERE related_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_employee_documents_s3_key
ON employee_documents(s3_key);

-- コメント追加
COMMENT ON COLUMN employee_documents.category IS 'ドキュメントカテゴリ: FACE_PHOTO, LICENSE_FRONT, LICENSE_BACK, QUALIFICATION_CERT, VEHICLE_INSPECTION, HEALTH_CHECKUP, APTITUDE_CHECKUP, OTHER';
COMMENT ON COLUMN employee_documents.s3_key IS 'S3オブジェクトキー（パス）';
COMMENT ON COLUMN employee_documents.s3_url IS 'S3オブジェクトのURL';
COMMENT ON COLUMN employee_documents.related_id IS '関連ID（資格ID、車両IDなど）';
COMMENT ON COLUMN employee_documents.uploaded_at IS 'アップロード日時（タイムゾーン付き）';
