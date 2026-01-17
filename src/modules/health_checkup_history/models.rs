use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// 健康診断履歴
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct HealthCheckupHistory {
    pub id: i32,
    pub employee_id: i32,
    pub health_checkup_type_id: i32,
    pub checkup_date: NaiveDate,
    pub expiration_date: Option<NaiveDate>,
    pub medical_institution: Option<String>,
    pub result: Option<String>,
    pub notes: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 健康診断履歴作成用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateHealthCheckupHistory {
    pub employee_id: i32,
    pub health_checkup_type_id: i32,
    pub checkup_date: NaiveDate,
    pub expiration_date: Option<NaiveDate>,
    pub medical_institution: Option<String>,
    pub result: Option<String>,
    pub notes: Option<String>,
    pub is_active: bool,
}

/// 健康診断履歴更新用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateHealthCheckupHistory {
    pub id: i32,
    pub employee_id: i32,
    pub health_checkup_type_id: i32,
    pub checkup_date: NaiveDate,
    pub expiration_date: Option<NaiveDate>,
    pub medical_institution: Option<String>,
    pub result: Option<String>,
    pub notes: Option<String>,
    pub is_active: bool,
}

/// 健康診断種別マスタ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct HealthCheckupType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub required_frequency_months: Option<i32>,
    pub is_mandatory: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
