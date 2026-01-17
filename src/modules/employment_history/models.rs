use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct EmploymentHistory {
    pub id: i32,
    pub employee_id: i32,
    pub company_name: String,
    pub department: Option<String>,
    pub position: Option<String>,
    pub job_description: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub is_current: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateEmploymentHistory {
    pub employee_id: i32,
    pub company_name: String,
    pub department: Option<String>,
    pub position: Option<String>,
    pub job_description: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub is_current: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateEmploymentHistory {
    pub id: i32,
    pub company_name: String,
    pub department: Option<String>,
    pub position: Option<String>,
    pub job_description: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub is_current: bool,
}
