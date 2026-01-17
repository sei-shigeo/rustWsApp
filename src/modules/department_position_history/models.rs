use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// 部署・役職・営業所履歴
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct DepartmentPositionHistory {
    pub id: i32,
    pub employee_id: i32,
    pub office_id: Option<i32>,
    pub department_id: Option<i32>,
    pub position_id: Option<i32>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub is_current: bool,
    pub change_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 部署・役職・営業所履歴作成用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateDepartmentPositionHistory {
    pub employee_id: i32,
    pub office_id: Option<i32>,
    pub department_id: Option<i32>,
    pub position_id: Option<i32>,
    pub start_date: NaiveDate,
    pub is_current: bool,
    pub change_reason: Option<String>,
}

/// 部署・役職・営業所履歴更新用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateDepartmentPositionHistory {
    pub id: i32,
    pub employee_id: i32,
    pub office_id: Option<i32>,
    pub department_id: Option<i32>,
    pub position_id: Option<i32>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub is_current: bool,
    pub change_reason: Option<String>,
}
