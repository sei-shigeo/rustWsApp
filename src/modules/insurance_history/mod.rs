pub mod handlers;
pub mod models;
#[cfg(feature = "server")]
mod repository;

pub use handlers::{
    create_insurance_history, delete_insurance_history, get_active_insurance_history,
    get_all_insurance_types, get_insurance_history_by_employee, update_insurance_history,
};
pub use models::{CreateInsuranceHistory, InsuranceHistory, InsuranceType, UpdateInsuranceHistory};
