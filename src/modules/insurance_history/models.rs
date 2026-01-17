use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// 保険証履歴
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct InsuranceHistory {
    pub id: i32,
    pub employee_id: i32,
    pub insurance_type_id: i32,
    pub insurance_number: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub insurer_name: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 保険証履歴作成用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateInsuranceHistory {
    pub employee_id: i32,
    pub insurance_type_id: i32,
    pub insurance_number: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub insurer_name: Option<String>,
    pub is_active: bool,
}

/// 保険証履歴更新用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateInsuranceHistory {
    pub id: i32,
    pub employee_id: i32,
    pub insurance_type_id: i32,
    pub insurance_number: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub insurer_name: Option<String>,
    pub is_active: bool,
}

/// 保険種別マスタ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct InsuranceType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub is_mandatory: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
