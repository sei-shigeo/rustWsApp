use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// 運転免許証
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct License {
    pub id: i32,
    pub employee_id: i32,
    pub license_type_id: i32,
    pub license_number: Option<String>,
    pub issue_date: Option<NaiveDate>,
    pub expiration_date: NaiveDate,
    pub issuing_authority: Option<String>,
    pub conditions: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 運転免許証作成用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateLicense {
    pub employee_id: i32,
    pub license_type_id: i32,
    pub license_number: Option<String>,
    pub issue_date: Option<NaiveDate>,
    pub expiration_date: NaiveDate,
    pub issuing_authority: Option<String>,
    pub conditions: Option<String>,
    pub is_active: bool,
}

/// 運転免許証更新用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateLicense {
    pub id: i32,
    pub employee_id: i32,
    pub license_type_id: i32,
    pub license_number: Option<String>,
    pub issue_date: Option<NaiveDate>,
    pub expiration_date: NaiveDate,
    pub issuing_authority: Option<String>,
    pub conditions: Option<String>,
    pub is_active: bool,
}

/// 免許種別マスタ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct LicenseType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub display_order: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
