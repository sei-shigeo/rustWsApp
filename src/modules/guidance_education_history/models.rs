#[cfg(feature = "server")]
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// 指導教育履歴
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct GuidanceEducationHistory {
    pub id: i32,
    pub employee_id: i32,
    pub guidance_education_type_id: i32,
    pub education_date: NaiveDate,
    pub expiration_date: Option<NaiveDate>,
    pub instructor_name: Option<String>,
    pub location: Option<String>,
    #[cfg(feature = "server")]
    pub duration_hours: Option<BigDecimal>,
    #[cfg(not(feature = "server"))]
    pub duration_hours: Option<f64>,
    pub content: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 指導教育履歴作成用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateGuidanceEducationHistory {
    pub employee_id: i32,
    pub guidance_education_type_id: i32,
    pub education_date: NaiveDate,
    pub expiration_date: Option<NaiveDate>,
    pub instructor_name: Option<String>,
    pub location: Option<String>,
    #[cfg(feature = "server")]
    pub duration_hours: Option<BigDecimal>,
    #[cfg(not(feature = "server"))]
    pub duration_hours: Option<f64>,
    pub content: Option<String>,
    pub notes: Option<String>,
}

/// 指導教育履歴更新用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateGuidanceEducationHistory {
    pub id: i32,
    pub employee_id: i32,
    pub guidance_education_type_id: i32,
    pub education_date: NaiveDate,
    pub expiration_date: Option<NaiveDate>,
    pub instructor_name: Option<String>,
    pub location: Option<String>,
    #[cfg(feature = "server")]
    pub duration_hours: Option<BigDecimal>,
    #[cfg(not(feature = "server"))]
    pub duration_hours: Option<f64>,
    pub content: Option<String>,
    pub notes: Option<String>,
}

/// 指導教育種別マスタ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct GuidanceEducationType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub required_frequency_months: Option<i32>,
    pub is_mandatory: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
