-- ============================================================
-- マスターテーブル群（従業員関連）
-- ============================================================

-- 在留カード種別マスタ
CREATE TABLE residence_card_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE, -- 永住者、定住者、技能実習、特定技能など
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 初期データ挿入
INSERT INTO residence_card_types (name) VALUES
('永住者'),
('定住者'),
('技能実習'),
('特定技能'),
('その他');

-- 免許種別マスタ
CREATE TABLE license_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE, -- 大型、中型、準中型、普通など
    description TEXT,
    display_order INTEGER, -- 表示順序
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 初期データ挿入
INSERT INTO license_types (name, display_order) VALUES
('大型', 1),
('中型', 2),
('準中型', 3),
('普通', 4),
('大型特殊', 5),
('けん引', 6);

-- 資格種別マスタ
CREATE TABLE qualification_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE, -- フォークリフト、玉掛けなど
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 初期データ挿入
INSERT INTO qualification_types (name) VALUES
('フォークリフト'),
('玉掛け'),
('クレーン'),
('危険物取扱者'),
('その他');

-- 保険種別マスタ
CREATE TABLE insurance_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE, -- 健康保険、厚生年金、雇用保険など
    description TEXT,
    is_mandatory BOOLEAN DEFAULT FALSE NOT NULL, -- 加入義務があるか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 初期データ挿入
INSERT INTO insurance_types (name, is_mandatory) VALUES
('健康保険', TRUE),
('厚生年金', TRUE),
('雇用保険', TRUE),
('労災保険', TRUE),
('その他', FALSE);

-- 指導教育種別マスタ
CREATE TABLE guidance_education_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL UNIQUE, -- 初任運転者講習、事故惹起者講習など
    description TEXT,
    required_frequency_months INTEGER, -- 必須実施頻度（月単位）
    is_mandatory BOOLEAN DEFAULT FALSE NOT NULL, -- 義務かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 初期データ挿入
INSERT INTO guidance_education_types (name, required_frequency_months, is_mandatory) VALUES
('初任運転者講習', NULL, TRUE),
('事故惹起者講習', NULL, TRUE),
('高齢運転者講習', 12, FALSE),
('健康起因事故防止講習', 12, FALSE),
('適齢診断', 12, FALSE),
('その他', NULL, FALSE);

-- 適性診断種別マスタ
CREATE TABLE aptitude_checkup_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE, -- 一般診断、初任診断、適齢診断など
    description TEXT,
    target_age_min INTEGER, -- 対象年齢（最小）
    target_age_max INTEGER, -- 対象年齢（最大）
    required_frequency_years INTEGER, -- 必須実施頻度（年単位）
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 初期データ挿入
INSERT INTO aptitude_checkup_types (name, target_age_min, target_age_max, required_frequency_years) VALUES
('一般診断', NULL, 64, 5),
('初任診断', NULL, NULL, NULL),
('適齢診断（65歳以上）', 65, NULL, 1),
('特定診断', NULL, NULL, NULL);

-- 健康診断種別マスタ
CREATE TABLE health_checkup_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE, -- 定期健康診断、雇入時健康診断など
    description TEXT,
    required_frequency_months INTEGER, -- 必須実施頻度（月単位）
    is_mandatory BOOLEAN DEFAULT FALSE NOT NULL, -- 義務かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 初期データ挿入
INSERT INTO health_checkup_types (name, required_frequency_months, is_mandatory) VALUES
('定期健康診断', 12, TRUE),
('雇入時健康診断', NULL, TRUE),
('特定健康診査（特定健診）', 12, FALSE),
('深夜業健康診断', 6, FALSE),
('その他', NULL, FALSE);

-- インデックス
CREATE INDEX idx_license_types_display_order ON license_types(display_order);
