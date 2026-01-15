-- ============================================================
-- 従業員基本情報テーブル
-- ============================================================

CREATE TABLE employees (
    -- 基本ID
    id SERIAL PRIMARY KEY,

    -- 所属会社
    company_id INTEGER
        REFERENCES companies(id) ON DELETE RESTRICT,

    -- 氏名情報（必須）
    first_name VARCHAR(100) NOT NULL, -- 名
    last_name VARCHAR(100) NOT NULL, -- 姓
    first_name_kana VARCHAR(100), -- 名（カナ）
    last_name_kana VARCHAR(100), -- 姓（カナ）
    legal_name VARCHAR(100), -- 法的名称（旧姓など）

    -- 国籍（デフォルトは日本、削除時はデフォルトに戻る）
    nationality_id INTEGER DEFAULT 1
        REFERENCES nationalities(id) ON DELETE SET DEFAULT,

    -- 個人情報
    birth_date DATE, -- 生年月日
    gender VARCHAR(10)
        CHECK (gender IN ('男性', '女性', 'その他')), -- 性別

    -- 連絡先情報
    email VARCHAR(100) UNIQUE, -- メールアドレス（重複不可）
    phone VARCHAR(20), -- 固定電話
    mobile VARCHAR(20), -- 携帯電話

    -- 雇用情報
    employee_code VARCHAR(50) NOT NULL UNIQUE, -- 社員番号
    start_date DATE, -- 雇用開始日
    end_date DATE, -- 雇用終了日（退職日）
    office_id INTEGER REFERENCES offices(id) ON DELETE SET NULL, -- 所属営業所
    department_id INTEGER REFERENCES departments(id) ON DELETE SET NULL, -- 所属部署
    position_id INTEGER REFERENCES positions(id) ON DELETE SET NULL, -- 役職

    -- ドライバー選任情報
    -- ドライバーとして正式に選任された期間を記録
    driver_start_date DATE, -- ドライバー選任日
    driver_end_date DATE, -- ドライバー選任解除日
    driver_end_note VARCHAR(255), -- 選任解除理由

    -- ステータス・メタ情報
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 在籍中かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- CHECK制約: 雇用開始日 < 雇用終了日
    CONSTRAINT chk_employees_employment_dates CHECK (
        end_date IS NULL OR start_date IS NULL OR start_date < end_date
    ),
    -- CHECK制約: ドライバー選任日 < 選任解除日
    CONSTRAINT chk_employees_driver_dates CHECK (
        driver_end_date IS NULL OR driver_start_date IS NULL OR driver_start_date < driver_end_date
    ),
    -- UNIQUE制約: 同一会社内で社員番号は重複不可（両方がNULLでない場合のみ）
    CONSTRAINT uq_employees_company_code UNIQUE NULLS NOT DISTINCT (company_id, employee_code)
);

-- 在留カード情報テーブル
CREATE TABLE residence_cards (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL
        REFERENCES employees(id) ON DELETE CASCADE,
    residence_card_type_id INTEGER NOT NULL
        REFERENCES residence_card_types(id) ON DELETE RESTRICT,
    card_number VARCHAR(50), -- 在留カード番号
    issue_date DATE, -- 発行日
    expiration_date DATE NOT NULL, -- 有効期限
    work_restrictions TEXT, -- 就労制限の有無・内容
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 現在有効かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- CHECK制約: 発行日 < 有効期限
    CONSTRAINT chk_residence_cards_dates CHECK (
        issue_date IS NULL OR issue_date < expiration_date
    )
);

-- 緊急連絡先テーブル
CREATE TABLE emergency_contacts (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL
        REFERENCES employees(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL, -- 氏名
    relationship VARCHAR(50), -- 続柄
    phone VARCHAR(20) NOT NULL, -- 電話番号
    mobile VARCHAR(20), -- 携帯電話
    postal_code VARCHAR(10), -- 郵便番号
    address VARCHAR(255), -- 住所
    priority_order INTEGER DEFAULT 1, -- 優先順位
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 現在有効かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 住所履歴テーブル
CREATE TABLE addresses (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL
        REFERENCES employees(id) ON DELETE CASCADE,
    postal_code VARCHAR(10) NOT NULL, -- 郵便番号
    prefecture VARCHAR(20) NOT NULL, -- 都道府県
    city VARCHAR(100) NOT NULL, -- 市区町村
    street VARCHAR(255) NOT NULL, -- 番地
    building VARCHAR(100), -- 建物名・部屋番号
    start_date DATE NOT NULL DEFAULT CURRENT_DATE, -- 居住開始日
    end_date DATE, -- 居住終了日
    is_current BOOLEAN DEFAULT TRUE NOT NULL, -- 現住所かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- CHECK制約: 居住開始日 < 居住終了日
    CONSTRAINT chk_addresses_dates CHECK (
        end_date IS NULL OR start_date < end_date
    )
);

-- 職歴テーブル
CREATE TABLE employment_history (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL
        REFERENCES employees(id) ON DELETE CASCADE,
    company_name VARCHAR(200) NOT NULL, -- 勤務先名
    department VARCHAR(100), -- 部署
    position VARCHAR(100), -- 役職
    job_description TEXT, -- 職務内容
    start_date DATE NOT NULL, -- 入社日
    end_date DATE, -- 退社日
    is_current BOOLEAN DEFAULT FALSE NOT NULL, -- 現在の職場かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- CHECK制約: 入社日 < 退社日
    CONSTRAINT chk_employment_history_dates CHECK (
        end_date IS NULL OR start_date < end_date
    )
);

-- 学歴テーブル
CREATE TABLE education_history (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL
        REFERENCES employees(id) ON DELETE CASCADE,
    school_name VARCHAR(200) NOT NULL, -- 学校名
    degree VARCHAR(100), -- 学位・卒業区分（中卒、高卒、大卒など）
    major VARCHAR(100), -- 専攻
    start_date DATE, -- 入学日
    end_date DATE, -- 卒業日
    graduation_status VARCHAR(50) CHECK (graduation_status IN ('卒業', '中退', '在学中')), -- 卒業状況
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- CHECK制約: 入学日 < 卒業日
    CONSTRAINT chk_education_history_dates CHECK (
        end_date IS NULL OR start_date IS NULL OR start_date < end_date
    )
);

-- 従業員銀行口座テーブル
CREATE TABLE employee_bank_accounts (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL
        REFERENCES employees(id) ON DELETE CASCADE,
    bank_code VARCHAR(4), -- 銀行コード（4桁）
    bank_name VARCHAR(100) NOT NULL, -- 銀行名
    branch_code VARCHAR(3), -- 支店コード（3桁）
    branch_name VARCHAR(100) NOT NULL, -- 支店名
    account_type VARCHAR(20) NOT NULL
        CHECK (account_type IN ('普通', '当座', '貯蓄')), -- 口座種別
    account_number VARCHAR(20) NOT NULL, -- 口座番号
    account_holder_name VARCHAR(100) NOT NULL, -- 口座名義人（カナ）
    is_primary BOOLEAN DEFAULT FALSE NOT NULL, -- 給与振込用のメイン口座かどうか
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 現在有効な口座かどうか
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 部署・役職・営業所履歴テーブル
CREATE TABLE department_position_history (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL
        REFERENCES employees(id) ON DELETE CASCADE,
    office_id INTEGER REFERENCES offices(id) ON DELETE SET NULL, -- 営業所
    department_id INTEGER REFERENCES departments(id) ON DELETE SET NULL, -- 部署
    position_id INTEGER REFERENCES positions(id) ON DELETE SET NULL, -- 役職
    start_date DATE NOT NULL DEFAULT CURRENT_DATE, -- 開始日
    end_date DATE, -- 終了日
    is_current BOOLEAN DEFAULT TRUE NOT NULL, -- 現在の配属かどうか
    change_reason VARCHAR(255), -- 異動理由
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- CHECK制約: 開始日 < 終了日
    CONSTRAINT chk_department_position_history_dates CHECK (
        end_date IS NULL OR start_date < end_date
    )
);

-- インデックス
CREATE INDEX idx_employees_company_id ON employees(company_id);
CREATE INDEX idx_employees_nationality_id ON employees(nationality_id);
CREATE INDEX idx_employees_office_id ON employees(office_id);
CREATE INDEX idx_employees_department_id ON employees(department_id);
CREATE INDEX idx_employees_position_id ON employees(position_id);
CREATE INDEX idx_employees_employee_code ON employees(employee_code);
CREATE INDEX idx_employees_active ON employees(is_active);
CREATE INDEX idx_employees_name ON employees(last_name, first_name);

CREATE INDEX idx_residence_cards_employee_id ON residence_cards(employee_id);
CREATE INDEX idx_residence_cards_expiration_date ON residence_cards(expiration_date) WHERE is_active = TRUE;
CREATE INDEX idx_residence_cards_active ON residence_cards(employee_id, is_active) WHERE is_active = TRUE;

CREATE INDEX idx_emergency_contacts_employee_id ON emergency_contacts(employee_id);
CREATE INDEX idx_emergency_contacts_active ON emergency_contacts(employee_id, is_active) WHERE is_active = TRUE;

CREATE INDEX idx_addresses_employee_id ON addresses(employee_id);
CREATE INDEX idx_addresses_current ON addresses(employee_id, is_current) WHERE is_current = TRUE;

CREATE INDEX idx_employment_history_employee_id ON employment_history(employee_id);
CREATE INDEX idx_education_history_employee_id ON education_history(employee_id);

CREATE INDEX idx_employee_bank_accounts_employee_id ON employee_bank_accounts(employee_id);
CREATE INDEX idx_employee_bank_accounts_active ON employee_bank_accounts(employee_id, is_active) WHERE is_active = TRUE;
-- 部分UNIQUE制約: 同一従業員のメイン口座は1つまで（部分ユニークインデックスとして実装）
CREATE UNIQUE INDEX uq_employee_bank_accounts_primary
ON employee_bank_accounts(employee_id)
WHERE (is_primary = TRUE AND is_active = TRUE);

CREATE INDEX idx_department_position_history_employee_id ON department_position_history(employee_id);
CREATE INDEX idx_department_position_history_current ON department_position_history(employee_id, is_current) WHERE is_current = TRUE;
