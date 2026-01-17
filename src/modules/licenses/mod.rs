pub mod handlers;
pub mod models;
#[cfg(feature = "server")]
mod repository;

pub use handlers::{
    create_license, delete_license, get_active_licenses, get_all_license_types,
    get_licenses_by_employee, update_license,
};
pub use models::{CreateLicense, License, LicenseType, UpdateLicense};
