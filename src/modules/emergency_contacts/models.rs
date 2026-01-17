use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct EmergencyContact {
    pub id: i32,
    pub employee_id: i32,
    pub name: String,
    pub relationship: Option<String>,
    pub phone: String,
    pub mobile: Option<String>,
    pub postal_code: Option<String>,
    pub address: Option<String>,
    pub priority_order: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateEmergencyContact {
    pub employee_id: i32,
    pub name: String,
    pub relationship: Option<String>,
    pub phone: String,
    pub mobile: Option<String>,
    pub postal_code: Option<String>,
    pub address: Option<String>,
    pub priority_order: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateEmergencyContact {
    pub id: i32,
    pub name: String,
    pub relationship: Option<String>,
    pub phone: String,
    pub mobile: Option<String>,
    pub postal_code: Option<String>,
    pub address: Option<String>,
    pub priority_order: i32,
    pub is_active: bool,
}
