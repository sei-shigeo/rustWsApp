#![allow(clippy::struct_field_names)]

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

// 従業員データのモデル（CardListItem用 - 基本情報のみ）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct Employee {
    pub id: i32,
    pub employee_code: String,
    pub first_name: String,
    pub last_name: String,
    pub mobile: Option<String>,
    pub nationality_id: Option<i32>,
    pub birth_date: Option<NaiveDate>,
    pub gender: Option<String>,
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

// 住所情報のモデル
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct Address {
    pub id: i32,
    pub employee_id: i32,
    pub postal_code: String,
    pub prefecture: String,
    pub city: String,
    pub street: String,
    pub building: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub is_current: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 住所情報を含む従業員モデル（カード表示用）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmployeeWithAddress {
    pub id: i32,
    pub employee_code: String,
    pub first_name: String,
    pub last_name: String,
    pub mobile: Option<String>,
    pub nationality_id: Option<i32>,
    pub birth_date: Option<NaiveDate>,
    pub gender: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // 現住所情報
    pub current_postal_code: Option<String>,
    pub current_prefecture: Option<String>,
    pub current_city: Option<String>,
    pub current_street: Option<String>,
    pub current_building: Option<String>,
}

// 住所作成用のリクエストモデル
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateAddress {
    pub employee_id: i32,
    pub postal_code: String,
    pub prefecture: String,
    pub city: String,
    pub street: String,
    pub building: Option<String>,
    pub start_date: NaiveDate,
    pub is_current: bool,
}

// 住所更新用のリクエストモデル
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateAddress {
    pub id: i32,
    pub employee_id: i32,
    pub postal_code: String,
    pub prefecture: String,
    pub city: String,
    pub street: String,
    pub building: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub is_current: bool,
}
