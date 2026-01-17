mod components;
mod handlers;
pub mod models;
mod page;

#[cfg(feature = "server")]
mod repository;
mod validation;

pub use components::EmployeeDetailPage;
pub use models::{
    Address, CreateAddress, Employee, EmployeeFull, EmployeeWithAddress, UpdateAddress,
};
pub use page::EmployeesPage;
