use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct ResidenceCard {
    pub id: i32,
    pub employee_id: i32,
    pub residence_card_type_id: i32,
    pub card_number: Option<String>,
    pub issue_date: Option<NaiveDate>,
    pub expiration_date: NaiveDate,
    pub work_restrictions: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateResidenceCard {
    pub employee_id: i32,
    pub residence_card_type_id: i32,
    pub card_number: Option<String>,
    pub issue_date: Option<NaiveDate>,
    pub expiration_date: NaiveDate,
    pub work_restrictions: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateResidenceCard {
    pub id: i32,
    pub residence_card_type_id: i32,
    pub card_number: Option<String>,
    pub issue_date: Option<NaiveDate>,
    pub expiration_date: NaiveDate,
    pub work_restrictions: Option<String>,
    pub is_active: bool,
}
