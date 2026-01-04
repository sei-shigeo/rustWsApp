use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// 従業員データのモデル
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 従業員作成のリクエストモデル
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateEmployee {
    pub first_name: String,
    pub last_name: String,
}

// 更新用のリクエストモデル
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateEmployee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
}
