use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// 資格証
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct Qualification {
    pub id: i32,
    pub employee_id: i32,
    pub qualification_type_id: i32,
    pub qualification_number: Option<String>,
    pub issue_date: Option<NaiveDate>,
    pub expiration_date: Option<NaiveDate>,
    pub issuing_authority: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 資格証作成用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateQualification {
    pub employee_id: i32,
    pub qualification_type_id: i32,
    pub qualification_number: Option<String>,
    pub issue_date: Option<NaiveDate>,
    pub expiration_date: Option<NaiveDate>,
    pub issuing_authority: Option<String>,
    pub is_active: bool,
}

/// 資格証更新用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateQualification {
    pub id: i32,
    pub employee_id: i32,
    pub qualification_type_id: i32,
    pub qualification_number: Option<String>,
    pub issue_date: Option<NaiveDate>,
    pub expiration_date: Option<NaiveDate>,
    pub issuing_authority: Option<String>,
    pub is_active: bool,
}

/// 資格種別マスタ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct QualificationType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
