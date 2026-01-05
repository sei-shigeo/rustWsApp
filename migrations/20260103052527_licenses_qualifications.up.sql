-- ============================================================
-- 免許・資格・健康診断関連テーブル群
-- ============================================================

-- 運転免許証テーブル
CREATE TABLE licenses (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL
        REFERENCES employees(id) ON DELETE CASCADE,
    license_type_id INTEGER NOT NULL
        REFERENCES license_types(id) ON DELETE RESTRICT,
    license_number VARCHAR(50), -- 免許証番号
    issue_date DATE, -- 交付日
    expiration_date DATE NOT NULL, -- 有効期限
    issuing_authority VARCHAR(100), -- 交付機関（公安委員会など）
    conditions TEXT, -- 条件（眼鏡等）
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 現在有効かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- CHECK制約: 交付日 < 有効期限
    CONSTRAINT chk_licenses_dates CHECK (
        issue_date IS NULL OR issue_date < expiration_date
    )
);

-- 資格証テーブル
CREATE TABLE qualifications (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL
        REFERENCES employees(id) ON DELETE CASCADE,
    qualification_type_id INTEGER NOT NULL
        REFERENCES qualification_types(id) ON DELETE RESTRICT,
    qualification_number VARCHAR(100), -- 資格証番号
    issue_date DATE, -- 取得日
    expiration_date DATE, -- 有効期限（無期限の場合はNULL）
    issuing_authority VARCHAR(100), -- 発行機関
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 現在有効かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- CHECK制約: 取得日 < 有効期限
    CONSTRAINT chk_qualifications_dates CHECK (
        expiration_date IS NULL OR issue_date IS NULL OR issue_date < expiration_date
    )
);

-- 保険証履歴テーブル
CREATE TABLE insurance_history (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL
        REFERENCES employees(id) ON DELETE CASCADE,
    insurance_type_id INTEGER NOT NULL
        REFERENCES insurance_types(id) ON DELETE RESTRICT,
    insurance_number VARCHAR(100), -- 保険証番号
    start_date DATE NOT NULL, -- 加入日
    end_date DATE, -- 終了日
    insurer_name VARCHAR(200), -- 保険者名
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 現在加入中かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- CHECK制約: 加入日 < 終了日
    CONSTRAINT chk_insurance_history_dates CHECK (
        end_date IS NULL OR start_date < end_date
    )
);

-- 健康診断履歴テーブル
CREATE TABLE health_checkup_history (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL
        REFERENCES employees(id) ON DELETE CASCADE,
    health_checkup_type_id INTEGER NOT NULL
        REFERENCES health_checkup_types(id) ON DELETE RESTRICT,
    checkup_date DATE NOT NULL, -- 受診日
    expiration_date DATE, -- 有効期限
    medical_institution VARCHAR(200), -- 受診医療機関
    result VARCHAR(50) CHECK (result IN ('異常なし', '要経過観察', '要精密検査', '要治療')), -- 判定結果
    notes TEXT, -- 所見・備考
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 現在有効かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- CHECK制約: 受診日 < 有効期限
    CONSTRAINT chk_health_checkup_dates CHECK (
        expiration_date IS NULL OR checkup_date < expiration_date
    )
);

-- 適性診断履歴テーブル
CREATE TABLE aptitude_checkup_history (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL
        REFERENCES employees(id) ON DELETE CASCADE,
    aptitude_checkup_type_id INTEGER NOT NULL
        REFERENCES aptitude_checkup_types(id) ON DELETE RESTRICT,
    checkup_date DATE NOT NULL, -- 受診日
    expiration_date DATE, -- 有効期限
    testing_organization VARCHAR(200), -- 実施機関
    result VARCHAR(50), -- 判定結果
    notes TEXT, -- 所見・備考
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 現在有効かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- CHECK制約: 受診日 < 有効期限
    CONSTRAINT chk_aptitude_checkup_dates CHECK (
        expiration_date IS NULL OR checkup_date < expiration_date
    )
);

-- 指導教育履歴テーブル
CREATE TABLE guidance_education_history (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL
        REFERENCES employees(id) ON DELETE CASCADE,
    guidance_education_type_id INTEGER NOT NULL
        REFERENCES guidance_education_types(id) ON DELETE RESTRICT,
    education_date DATE NOT NULL, -- 実施日
    expiration_date DATE, -- 有効期限
    instructor_name VARCHAR(100), -- 指導者名
    location VARCHAR(200), -- 実施場所
    duration_hours DECIMAL(5, 2), -- 実施時間（時間）
    content TEXT, -- 指導内容
    notes TEXT, -- 備考
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- CHECK制約: 実施日 < 有効期限
    CONSTRAINT chk_guidance_education_dates CHECK (
        expiration_date IS NULL OR education_date < expiration_date
    )
);

-- 書類種別マスタ
CREATE TABLE document_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE, -- 履歴書、契約書、誓約書など
    description TEXT,
    is_mandatory BOOLEAN DEFAULT FALSE NOT NULL, -- 必須書類かどうか
    retention_years INTEGER, -- 保管期間（年）
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 初期データ挿入
INSERT INTO document_types (name, is_mandatory, retention_years) VALUES
('履歴書', TRUE, 5),
('雇用契約書', TRUE, 7),
('誓約書', TRUE, 7),
('身分証明書コピー', TRUE, 5),
('免許証コピー', FALSE, NULL),
('資格証コピー', FALSE, NULL),
('その他', FALSE, NULL);

-- 従業員書類テーブル
CREATE TABLE employee_documents (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL
        REFERENCES employees(id) ON DELETE CASCADE,
    document_type_id INTEGER NOT NULL
        REFERENCES document_types(id) ON DELETE RESTRICT,
    file_name VARCHAR(255) NOT NULL, -- ファイル名
    file_path VARCHAR(500) NOT NULL, -- ファイルパス（クラウドストレージのURLなど）
    file_size BIGINT, -- ファイルサイズ（バイト）
    mime_type VARCHAR(100), -- MIMEタイプ
    upload_date DATE NOT NULL DEFAULT CURRENT_DATE, -- アップロード日
    expiration_date DATE, -- 有効期限（ある場合）
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 現在有効かどうか
    notes TEXT, -- 備考
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- インデックス
CREATE INDEX idx_licenses_employee_id ON licenses(employee_id);
CREATE INDEX idx_licenses_license_type_id ON licenses(license_type_id);
CREATE INDEX idx_licenses_expiration_date ON licenses(expiration_date) WHERE is_active = TRUE;
CREATE INDEX idx_licenses_active ON licenses(employee_id, is_active) WHERE is_active = TRUE;

CREATE INDEX idx_qualifications_employee_id ON qualifications(employee_id);
CREATE INDEX idx_qualifications_qualification_type_id ON qualifications(qualification_type_id);
CREATE INDEX idx_qualifications_expiration_date ON qualifications(expiration_date) WHERE is_active = TRUE;
CREATE INDEX idx_qualifications_active ON qualifications(employee_id, is_active) WHERE is_active = TRUE;

CREATE INDEX idx_insurance_history_employee_id ON insurance_history(employee_id);
CREATE INDEX idx_insurance_history_insurance_type_id ON insurance_history(insurance_type_id);
CREATE INDEX idx_insurance_history_active ON insurance_history(employee_id, is_active) WHERE is_active = TRUE;

CREATE INDEX idx_health_checkup_history_employee_id ON health_checkup_history(employee_id);
CREATE INDEX idx_health_checkup_history_type_id ON health_checkup_history(health_checkup_type_id);
CREATE INDEX idx_health_checkup_expiration_date ON health_checkup_history(expiration_date) WHERE is_active = TRUE;

CREATE INDEX idx_aptitude_checkup_history_employee_id ON aptitude_checkup_history(employee_id);
CREATE INDEX idx_aptitude_checkup_history_type_id ON aptitude_checkup_history(aptitude_checkup_type_id);
CREATE INDEX idx_aptitude_checkup_expiration_date ON aptitude_checkup_history(expiration_date) WHERE is_active = TRUE;

CREATE INDEX idx_guidance_education_history_employee_id ON guidance_education_history(employee_id);
CREATE INDEX idx_guidance_education_history_type_id ON guidance_education_history(guidance_education_type_id);

CREATE INDEX idx_employee_documents_employee_id ON employee_documents(employee_id);
CREATE INDEX idx_employee_documents_type ON employee_documents(document_type_id, is_active) WHERE is_active = TRUE;
CREATE INDEX idx_employee_documents_active ON employee_documents(employee_id, is_active) WHERE is_active = TRUE;
