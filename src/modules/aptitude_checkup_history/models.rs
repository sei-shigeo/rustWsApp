use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// 適性診断履歴
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct AptitudeCheckupHistory {
    pub id: i32,
    pub employee_id: i32,
    pub aptitude_checkup_type_id: i32,
    pub checkup_date: NaiveDate,
    pub expiration_date: Option<NaiveDate>,
    pub testing_organization: Option<String>,
    pub result: Option<String>,
    pub notes: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 適性診断履歴作成用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateAptitudeCheckupHistory {
    pub employee_id: i32,
    pub aptitude_checkup_type_id: i32,
    pub checkup_date: NaiveDate,
    pub expiration_date: Option<NaiveDate>,
    pub testing_organization: Option<String>,
    pub result: Option<String>,
    pub notes: Option<String>,
    pub is_active: bool,
}

/// 適性診断履歴更新用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateAptitudeCheckupHistory {
    pub id: i32,
    pub employee_id: i32,
    pub aptitude_checkup_type_id: i32,
    pub checkup_date: NaiveDate,
    pub expiration_date: Option<NaiveDate>,
    pub testing_organization: Option<String>,
    pub result: Option<String>,
    pub notes: Option<String>,
    pub is_active: bool,
}

/// 適性診断種別マスタ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct AptitudeCheckupType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub target_age_min: Option<i32>,
    pub target_age_max: Option<i32>,
    pub required_frequency_years: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
