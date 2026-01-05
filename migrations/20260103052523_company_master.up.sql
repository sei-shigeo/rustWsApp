-- ============================================================
-- 会社・営業所・部署マスター
-- ============================================================

-- 会社マスタ
-- 会社の基本情報・法的情報を管理
CREATE TABLE companies (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL, -- 会社名
    name_kana VARCHAR(200), -- 会社名カナ
    corporate_number VARCHAR(13) UNIQUE, -- 法人番号（13桁）
    invoice_registration_number VARCHAR(14) UNIQUE, -- 適格請求書発行事業者登録番号（T+13桁）
    representative_name VARCHAR(100), -- 代表者名
    postal_code VARCHAR(10), -- 本社郵便番号
    address VARCHAR(255), -- 本社住所
    phone VARCHAR(20), -- 電話番号
    fax VARCHAR(20), -- FAX番号
    email VARCHAR(255), -- メールアドレス
    establishment_date DATE, -- 設立日
    capital_amount BIGINT, -- 資本金（円）
    fiscal_year_end_month INTEGER CHECK (fiscal_year_end_month BETWEEN 1 AND 12), -- 決算月（1-12）
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 稼働中かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);
INSERT INTO companies ( name ) VALUES ( '株式会社和清商事');

-- 会社銀行口座テーブル
-- 会社の銀行口座情報を管理（給与振込元、経費精算用など）
CREATE TABLE company_bank_accounts (
    id SERIAL PRIMARY KEY,
    company_id INTEGER NOT NULL
        REFERENCES companies(id) ON DELETE CASCADE, -- 会社削除時にカスケード削除
    bank_code VARCHAR(4), -- 銀行コード（4桁、銀行APIから取得）
    bank_name VARCHAR(100) NOT NULL, -- 銀行名
    branch_code VARCHAR(3), -- 支店コード（3桁、銀行APIから取得）
    branch_name VARCHAR(100) NOT NULL, -- 支店名
    account_type VARCHAR(20) NOT NULL
        CHECK (account_type IN ('普通', '当座', '貯蓄')), -- 口座種別
    account_number VARCHAR(20) NOT NULL, -- 口座番号
    account_holder_name VARCHAR(100) NOT NULL, -- 口座名義人（カナ）
    purpose VARCHAR(100), -- 用途（給与振込用、経費精算用など）
    is_primary BOOLEAN DEFAULT FALSE NOT NULL, -- メイン口座かどうか
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 現在有効な口座かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 国籍マスタ
-- 従業員の国籍を管理するテーブル
CREATE TABLE nationalities (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE, -- 国籍名の重複を防ぐためUNIQUE制約を追加
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 初期データ挿入
INSERT INTO nationalities (name) VALUES
('日本'),
('ブラジル'),
('ペルー');

-- 営業所（事業所）マスタ
-- 本社や支店、営業所などの拠点を管理
CREATE TABLE offices (
    id SERIAL PRIMARY KEY,
    company_id INTEGER NOT NULL
        REFERENCES companies(id) ON DELETE RESTRICT, -- 会社（会社の削除を防ぐ）
    name VARCHAR(100) NOT NULL, -- 営業所名
    office_code VARCHAR(50), -- 営業所コード
    postal_code VARCHAR(10), -- 郵便番号
    address VARCHAR(255), -- 住所
    phone VARCHAR(20), -- 電話番号
    is_headquarters BOOLEAN DEFAULT FALSE NOT NULL, -- 本社かどうか
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 稼働中かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- UNIQUE制約: 同一会社内で営業所名は重複不可
    CONSTRAINT uq_offices_company_name UNIQUE (company_id, name),
    -- UNIQUE制約: 同一会社内で営業所コードは重複不可（NULLは許可）
    CONSTRAINT uq_offices_company_code UNIQUE (company_id, office_code)
);
INSERT INTO offices (company_id, name) VALUES(1, '本社営業所');

-- 部署マスタ
-- 会社の部署情報を管理
CREATE TABLE departments (
    id SERIAL PRIMARY KEY,
    company_id INTEGER NOT NULL
        REFERENCES companies(id) ON DELETE RESTRICT, -- 会社（会社の削除を防ぐ）
    name VARCHAR(100) NOT NULL, -- 部署名
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- UNIQUE制約: 同一会社内で部署名は重複不可
    CONSTRAINT uq_departments_company_name UNIQUE (company_id, name)
);

-- 役職マスタ
-- 課長、部長、主任などの役職情報
CREATE TABLE positions (
    id SERIAL PRIMARY KEY,
    company_id INTEGER NOT NULL
        REFERENCES companies(id) ON DELETE RESTRICT, -- 会社
    name VARCHAR(100) NOT NULL, -- 役職名
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- UNIQUE制約: 同一会社内で役職名は重複不可
    CONSTRAINT uq_positions_company_name UNIQUE (company_id, name)
);

-- インデックス
CREATE INDEX idx_company_bank_accounts_company_id ON company_bank_accounts(company_id);
CREATE INDEX idx_offices_company_id ON offices(company_id);
CREATE INDEX idx_departments_company_id ON departments(company_id);
CREATE INDEX idx_positions_company_id ON positions(company_id);

-- 部分UNIQUE制約: 同一会社のメイン口座は1つまで（部分ユニークインデックスとして実装）
CREATE UNIQUE INDEX uq_company_bank_accounts_primary
ON company_bank_accounts(company_id)
WHERE (is_primary = TRUE AND is_active = TRUE);
