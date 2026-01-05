-- ============================================================
-- 車両管理テーブル群
-- ============================================================

-- 車両メーカーマスタ
CREATE TABLE vehicle_manufacturers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE, -- いすゞ、日野、三菱ふそう、UDトラックスなど
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 初期データ挿入
INSERT INTO vehicle_manufacturers (name) VALUES
('いすゞ'),
('日野'),
('三菱ふそう'),
('UDトラックス'),
('その他');

-- 車両種別マスタ
CREATE TABLE vehicle_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE, -- 小型トラック、中型トラック、大型トラック、トレーラーなど
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 初期データ挿入
INSERT INTO vehicle_types (name) VALUES
('小型トラック'),
('中型トラック'),
('大型トラック'),
('トレーラー'),
('その他');

-- 車両所有者種別マスタ
CREATE TABLE vehicle_ownership_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE, -- 自社保有、リース、レンタルなど
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 初期データ挿入
INSERT INTO vehicle_ownership_types (name) VALUES
('自社保有'),
('リース'),
('レンタル');

-- 車両マスタ
CREATE TABLE vehicles (
    id SERIAL PRIMARY KEY,
    company_id INTEGER NOT NULL REFERENCES companies(id) ON DELETE RESTRICT,
    office_id INTEGER REFERENCES offices(id) ON DELETE SET NULL, -- 配属営業所
    vehicle_type_id INTEGER NOT NULL REFERENCES vehicle_types(id) ON DELETE RESTRICT,
    ownership_type_id INTEGER NOT NULL REFERENCES vehicle_ownership_types(id) ON DELETE RESTRICT,
    manufacturer_id INTEGER REFERENCES vehicle_manufacturers(id) ON DELETE SET NULL,

    -- 車両識別情報
    vehicle_number VARCHAR(50) NOT NULL, -- 車両番号（ナンバープレート）
    vehicle_code VARCHAR(50) UNIQUE, -- 社内管理コード

    -- 車両基本情報
    model VARCHAR(100), -- 車種・型式
    year INTEGER, -- 年式
    color VARCHAR(50), -- 色
    chassis_number VARCHAR(100), -- 車台番号

    -- 車検・保険情報
    inspection_expiration_date DATE, -- 車検有効期限
    insurance_expiration_date DATE, -- 自賠責保険有効期限
    voluntary_insurance_expiration_date DATE, -- 任意保険有効期限

    -- リース情報（リース車両の場合）
    lease_company_id INTEGER REFERENCES clients(id) ON DELETE SET NULL, -- リース会社
    lease_start_date DATE, -- リース開始日
    lease_end_date DATE, -- リース終了日
    monthly_lease_fee DECIMAL(10, 2), -- 月額リース料

    -- 購入情報（自社保有の場合）
    purchase_date DATE, -- 購入日
    purchase_price DECIMAL(12, 2), -- 購入金額

    -- 稼働状況
    is_active BOOLEAN DEFAULT TRUE NOT NULL, -- 稼働中かどうか
    retirement_date DATE, -- 廃車日
    retirement_reason TEXT, -- 廃車理由

    notes TEXT, -- 備考
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- CHECK制約: リース期間の整合性
    CONSTRAINT chk_vehicles_lease_dates CHECK (
        lease_end_date IS NULL OR lease_start_date IS NULL OR lease_start_date < lease_end_date
    ),
    -- UNIQUE制約: 同一会社内で車両番号は重複不可
    CONSTRAINT uq_vehicles_company_number UNIQUE (company_id, vehicle_number)
);

-- 車両点検履歴テーブル
CREATE TABLE vehicle_inspection_history (
    id SERIAL PRIMARY KEY,
    vehicle_id INTEGER NOT NULL REFERENCES vehicles(id) ON DELETE CASCADE,
    inspection_type VARCHAR(100) NOT NULL, -- 法定12ヶ月点検、法定24ヶ月点検（車検）、日常点検など
    inspection_date DATE NOT NULL, -- 点検日
    next_inspection_date DATE, -- 次回点検予定日
    inspector_name VARCHAR(100), -- 点検者名
    inspection_location VARCHAR(200), -- 点検場所（整備工場名など）
    cost DECIMAL(10, 2), -- 費用
    notes TEXT, -- 点検内容・結果
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- CHECK制約: 点検日 < 次回点検予定日
    CONSTRAINT chk_vehicle_inspection_dates CHECK (
        next_inspection_date IS NULL OR inspection_date < next_inspection_date
    )
);

-- 車両修理履歴テーブル
CREATE TABLE vehicle_repair_history (
    id SERIAL PRIMARY KEY,
    vehicle_id INTEGER NOT NULL REFERENCES vehicles(id) ON DELETE CASCADE,
    repair_date DATE NOT NULL, -- 修理日
    repair_type VARCHAR(100), -- 修理種別（定期整備、故障修理、事故修理など）
    repair_location VARCHAR(200), -- 修理場所（整備工場名など）
    repair_description TEXT NOT NULL, -- 修理内容
    cost DECIMAL(10, 2), -- 修理費用
    parts_replaced TEXT, -- 交換部品
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- インデックス
CREATE INDEX idx_vehicles_company_id ON vehicles(company_id);
CREATE INDEX idx_vehicles_office_id ON vehicles(office_id);
CREATE INDEX idx_vehicles_vehicle_type_id ON vehicles(vehicle_type_id);
CREATE INDEX idx_vehicles_ownership_type_id ON vehicles(ownership_type_id);
CREATE INDEX idx_vehicles_manufacturer_id ON vehicles(manufacturer_id);
CREATE INDEX idx_vehicles_active ON vehicles(is_active);
CREATE INDEX idx_vehicles_inspection_expiration ON vehicles(inspection_expiration_date) WHERE is_active = TRUE;
CREATE INDEX idx_vehicles_insurance_expiration ON vehicles(insurance_expiration_date) WHERE is_active = TRUE;
CREATE INDEX idx_vehicles_voluntary_insurance_expiration ON vehicles(voluntary_insurance_expiration_date) WHERE is_active = TRUE;

CREATE INDEX idx_vehicle_inspection_history_vehicle_id ON vehicle_inspection_history(vehicle_id);
CREATE INDEX idx_vehicle_inspection_history_date ON vehicle_inspection_history(inspection_date);
CREATE INDEX idx_vehicle_inspection_history_next_date ON vehicle_inspection_history(next_inspection_date);

CREATE INDEX idx_vehicle_repair_history_vehicle_id ON vehicle_repair_history(vehicle_id);
CREATE INDEX idx_vehicle_repair_history_date ON vehicle_repair_history(repair_date);
