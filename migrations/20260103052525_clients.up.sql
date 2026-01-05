-- ============================================================
-- 取引先管理テーブル群
-- ============================================================

-- 取引先種別マスタ
CREATE TABLE client_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE, -- 荷主、協力会社、リース会社など
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 初期データ挿入
INSERT INTO client_types (name) VALUES
('荷主'),
('協力会社'),
('リース会社'),
('整備工場'),
('その他');

-- 取引先マスタ
CREATE TABLE clients (
    id SERIAL PRIMARY KEY,
    company_id INTEGER NOT NULL
        REFERENCES companies(id) ON DELETE RESTRICT, -- 自社の会社ID
    client_type_id INTEGER NOT NULL
        REFERENCES client_types(id) ON DELETE RESTRICT,
    name VARCHAR(200) NOT NULL, -- 取引先名
    name_kana VARCHAR(200), -- 取引先名カナ
    corporate_number VARCHAR(13), -- 法人番号（13桁）
    invoice_registration_number VARCHAR(14), -- 適格請求書発行事業者登録番号（T+13桁）
    postal_code VARCHAR(10), -- 郵便番号
    address VARCHAR(255), -- 住所
    phone VARCHAR(20), -- 電話番号
    fax VARCHAR(20), -- FAX番号
    email VARCHAR(255), -- メールアドレス
    website_url VARCHAR(255), -- ウェブサイトURL
    payment_terms VARCHAR(100), -- 支払条件（月末締め翌月末払いなど）
    credit_limit DECIMAL(15, 2), -- 与信限度額
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 取引中かどうか
    notes TEXT, -- 備考
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- UNIQUE制約: 同一会社内で取引先名は重複不可
    CONSTRAINT uq_clients_company_name UNIQUE (company_id, name)
);

-- 取引先営業所テーブル
CREATE TABLE client_offices (
    id SERIAL PRIMARY KEY,
    client_id INTEGER NOT NULL
        REFERENCES clients(id) ON DELETE CASCADE,
    name VARCHAR(200) NOT NULL, -- 営業所名
    postal_code VARCHAR(10), -- 郵便番号
    address VARCHAR(255), -- 住所
    phone VARCHAR(20), -- 電話番号
    fax VARCHAR(20), -- FAX番号
    is_main_office BOOLEAN DEFAULT FALSE NOT NULL, -- 本社かどうか
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 稼働中かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- UNIQUE制約: 同一取引先内で営業所名は重複不可
    CONSTRAINT uq_client_offices_name UNIQUE (client_id, name)
);

-- 取引先担当者テーブル
CREATE TABLE client_contacts (
    id SERIAL PRIMARY KEY,
    client_id INTEGER NOT NULL
        REFERENCES clients(id) ON DELETE CASCADE,
    client_office_id INTEGER
        REFERENCES client_offices(id) ON DELETE SET NULL, -- 所属営業所
    name VARCHAR(100) NOT NULL, -- 担当者名
    name_kana VARCHAR(100), -- 担当者名カナ
    department VARCHAR(100), -- 部署
    position VARCHAR(100), -- 役職
    phone VARCHAR(20), -- 電話番号
    mobile VARCHAR(20), -- 携帯電話
    email VARCHAR(255), -- メールアドレス
    is_primary BOOLEAN DEFAULT FALSE NOT NULL, -- メイン担当者かどうか
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 現在有効かどうか
    notes TEXT, -- 備考
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- インデックス
CREATE INDEX idx_clients_company_id ON clients(company_id);
CREATE INDEX idx_clients_client_type_id ON clients(client_type_id);
CREATE INDEX idx_clients_active ON clients(is_active);
CREATE INDEX idx_client_offices_client_id ON client_offices(client_id);
CREATE INDEX idx_client_contacts_client_id ON client_contacts(client_id);
CREATE INDEX idx_client_contacts_client_office_id ON client_contacts(client_office_id);
