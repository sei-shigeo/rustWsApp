#![allow(clippy::struct_field_names)]

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

// 従業員データのモデル（簡易版 - 基本情報のみ）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct Employee {
    pub id: i32,
    pub employee_code: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 従業員の全詳細情報（必要に応じて使う）
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct EmployeeFull {
    pub id: i32,
    pub company_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub first_name_kana: Option<String>,
    pub last_name_kana: Option<String>,
    pub legal_name: Option<String>,
    pub nationality_id: Option<i32>,
    pub birth_date: Option<NaiveDate>,
    pub gender: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub employee_code: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub office_id: Option<i32>,
    pub department_id: Option<i32>,
    pub position_id: Option<i32>,
    pub driver_start_date: Option<NaiveDate>,
    pub driver_end_date: Option<NaiveDate>,
    pub driver_end_note: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 従業員作成のリクエストモデル
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateEmployee {
    pub employee_code: String,
    pub first_name: String,
    pub last_name: String,
}

// 更新用のリクエストモデル
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateEmployee {
    pub id: i32,
    pub employee_code: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
}
