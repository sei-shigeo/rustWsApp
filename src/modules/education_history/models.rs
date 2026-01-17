use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct EducationHistory {
    pub id: i32,
    pub employee_id: i32,
    pub school_name: String,
    pub degree: Option<String>,
    pub major: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub graduation_status: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateEducationHistory {
    pub employee_id: i32,
    pub school_name: String,
    pub degree: Option<String>,
    pub major: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub graduation_status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateEducationHistory {
    pub id: i32,
    pub school_name: String,
    pub degree: Option<String>,
    pub major: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub graduation_status: Option<String>,
}
