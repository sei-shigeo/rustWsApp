-- ============================================================
-- マスターテーブル群（従業員関連） ロールバック
-- ============================================================

-- インデックスを削除
DROP INDEX IF EXISTS idx_license_types_display_order;

-- テーブルを逆順で削除（外部キー制約を考慮）
DROP TABLE IF EXISTS health_checkup_types;
DROP TABLE IF EXISTS aptitude_checkup_types;
DROP TABLE IF EXISTS guidance_education_types;
DROP TABLE IF EXISTS insurance_types;
DROP TABLE IF EXISTS qualification_types;
DROP TABLE IF EXISTS license_types;
DROP TABLE IF EXISTS residence_card_types;
