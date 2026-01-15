mod handlers;
mod models;
mod page;

#[cfg(feature = "server")]
mod repository;
mod validation;

pub use page::EmployeesPage;
