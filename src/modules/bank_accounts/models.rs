use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct BankAccount {
    pub id: i32,
    pub employee_id: i32,
    pub bank_code: Option<String>,
    pub bank_name: String,
    pub branch_code: Option<String>,
    pub branch_name: String,
    pub account_type: String,
    pub account_number: String,
    pub account_holder_name: String,
    pub is_primary: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateBankAccount {
    pub employee_id: i32,
    pub bank_code: Option<String>,
    pub bank_name: String,
    pub branch_code: Option<String>,
    pub branch_name: String,
    pub account_type: String,
    pub account_number: String,
    pub account_holder_name: String,
    pub is_primary: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateBankAccount {
    pub id: i32,
    pub bank_code: Option<String>,
    pub bank_name: String,
    pub branch_code: Option<String>,
    pub branch_name: String,
    pub account_type: String,
    pub account_number: String,
    pub account_holder_name: String,
    pub is_primary: bool,
    pub is_active: bool,
}
